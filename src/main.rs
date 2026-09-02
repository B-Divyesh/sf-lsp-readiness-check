use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use lsp_readiness_check::{
    CheckStatus, SignedPacket, demo_payload, inspect_repository, load_or_create_signing_key, sign,
    verify,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser)]
#[command(name = "lsp-readiness", version, about = "Verify language tooling before an agent edits a repository", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe language servers, formatters, and the repository test command
    Check {
        /// Repository root to inspect
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Capability packet output path
        #[arg(short, long, default_value = ".lsp-readiness.json")]
        output: PathBuf,
        /// Ed25519 private key path; created on first use
        #[arg(long, default_value = ".lsp-readiness/signing.key")]
        key: PathBuf,
        /// Run the detected test command. This may take time.
        #[arg(long)]
        run_tests: bool,
        /// Print only the signed JSON packet
        #[arg(long)]
        json: bool,
    },
    /// Run the probe inside a locked-down Docker or Podman container
    Container {
        /// Repository root copied into the disposable container
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Pinned development image containing the repository tools
        #[arg(long)]
        image: String,
        /// Container runtime command
        #[arg(long, default_value = "docker")]
        runtime: String,
        /// Capability packet output path on the host
        #[arg(short, long, default_value = ".lsp-readiness.json")]
        output: PathBuf,
        /// Ed25519 private key path on the host; created on first use
        #[arg(long, default_value = ".lsp-readiness/signing.key")]
        key: PathBuf,
        /// Run the detected test command inside the container
        #[arg(long)]
        run_tests: bool,
        /// Print only the signed JSON packet
        #[arg(long)]
        json: bool,
    },
    /// Run a deterministic probe against bundled sample repository data
    Demo {
        /// Write the sample packet into a new temporary directory
        #[arg(long)]
        json: bool,
    },
    /// Verify an Ed25519 signature in a capability packet
    Verify {
        /// Signed capability packet
        packet: PathBuf,
        /// Print the verification result as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("lsp-readiness: {error:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<ExitCode> {
    match Cli::parse().command {
        Commands::Check {
            path,
            output,
            key,
            run_tests,
            json,
        } => {
            let payload = inspect_repository(&path, run_tests)?;
            let ready = payload.ready;
            let key = load_or_create_signing_key(&key)?;
            let packet = sign(payload, &key)?;
            let encoded = serde_json::to_string_pretty(&packet)?;
            if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
                fs::create_dir_all(parent)?;
            }
            fs::write(&output, format!("{encoded}\n"))
                .with_context(|| format!("cannot write {}", output.display()))?;
            if json {
                println!("{encoded}");
            } else {
                print_report(&packet);
                println!("\nSigned packet: {}", output.display());
            }
            Ok(if ready {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            })
        }
        Commands::Demo { json } => {
            let dir =
                std::env::temp_dir().join(format!("lsp-readiness-demo-{}", std::process::id()));
            fs::create_dir_all(&dir)?;
            let key = load_or_create_signing_key(&dir.join("signing.key"))?;
            let packet = sign(demo_payload(), &key)?;
            let output = dir.join("lsp-readiness.json");
            fs::write(
                &output,
                format!("{}\n", serde_json::to_string_pretty(&packet)?),
            )?;
            if json {
                println!("{}", serde_json::to_string_pretty(&packet)?);
            } else {
                print_report(&packet);
                println!("\nDemo — sample data, nothing is saved to your repository.");
                println!("Signed packet: {}", output.display());
            }
            Ok(ExitCode::SUCCESS)
        }
        Commands::Container {
            path,
            image,
            runtime,
            output,
            key,
            run_tests,
            json,
        } => run_container(&path, &image, &runtime, &output, &key, run_tests, json),
        Commands::Verify { packet, json } => {
            let packet: SignedPacket = serde_json::from_slice(&fs::read(&packet)?)?;
            verify(&packet)?;
            if json {
                println!("{{\"valid\":true,\"algorithm\":\"Ed25519\"}}");
            } else {
                println!("Valid Ed25519 signature for {}", packet.payload.repository);
            }
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn run_container(
    path: &Path,
    image: &str,
    runtime: &str,
    output: &Path,
    key: &Path,
    run_tests: bool,
    json: bool,
) -> Result<ExitCode> {
    let repository = path
        .canonicalize()
        .with_context(|| format!("cannot open {}", path.display()))?;
    let executable = std::env::current_exe()?.canonicalize()?;
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let staging = std::env::temp_dir().join(format!(
        "lsp-readiness-container-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&staging)?;
    if key.exists() {
        fs::copy(key, staging.join("signing.key"))?;
    }

    let mut command = Command::new(runtime);
    command.args([
        "run",
        "--rm",
        "--network",
        "none",
        "--read-only",
        "--cap-drop=ALL",
        "--security-opt=no-new-privileges",
        "--tmpfs",
        "/workspace:rw,exec,nosuid,size=1g",
        "--tmpfs",
        "/tmp:rw,noexec,nosuid,size=256m",
        "-e",
        "HOME=/tmp",
        "-v",
    ]);
    command.arg(format!("{}:/source:ro", repository.display()));
    command.arg("-v");
    command.arg(format!("{}:/out:rw", staging.display()));
    command.arg("-v");
    command.arg(format!(
        "{}:/usr/local/bin/lsp-readiness:ro",
        executable.display()
    ));
    command.args([image, "/bin/sh", "-c"]);
    command.arg(if run_tests {
        "cp -R /source/. /workspace && exec /usr/local/bin/lsp-readiness check /workspace --output /out/packet.json --key /out/signing.key --run-tests"
    } else {
        "cp -R /source/. /workspace && exec /usr/local/bin/lsp-readiness check /workspace --output /out/packet.json --key /out/signing.key"
    });
    let status = command.status().with_context(|| {
        format!("cannot start {runtime}; install it or choose --runtime podman")
    })?;

    let staged_packet = staging.join("packet.json");
    if !staged_packet.exists() {
        let _ = fs::remove_dir_all(&staging);
        if status.success() {
            anyhow::bail!("container finished without a capability packet");
        }
        return Ok(ExitCode::from(status.code().unwrap_or(2) as u8));
    }
    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = key.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    fs::copy(&staged_packet, output)?;
    if !key.exists() {
        fs::copy(staging.join("signing.key"), key)?;
    }
    let packet: SignedPacket = serde_json::from_slice(&fs::read(output)?)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&packet)?);
    } else {
        print_report(&packet);
        println!("\nIsolated probe: {runtime} / network disabled / source copied");
        println!("Signed packet: {}", output.display());
    }
    let _ = fs::remove_dir_all(&staging);
    Ok(if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(status.code().unwrap_or(1) as u8)
    })
}

fn print_report(packet: &SignedPacket) {
    println!("LSP READINESS / {}", packet.payload.repository);
    println!(
        "{}",
        if packet.payload.ready {
            "READY — agent edits may start"
        } else {
            "NOT READY — fix failed checks first"
        }
    );
    for capability in &packet.payload.capabilities {
        let mark = match capability.status {
            CheckStatus::Ready => "PASS",
            CheckStatus::Declared => "INFO",
            CheckStatus::Missing => "MISS",
            CheckStatus::Failed => "FAIL",
        };
        println!(
            "[{mark}] {:<11} {:<34} {}",
            capability.kind, capability.name, capability.evidence
        );
    }
    println!("Signature: Ed25519 / {}…", &packet.public_key[..12]);
}
