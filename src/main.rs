use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use lsp_readiness_check::{
    CheckStatus, ProbePayload, SignedPacket, inspect_demo_fixture, inspect_repository,
    load_or_create_signing_key, sign, verify,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

#[derive(Parser)]
#[command(name = "lsp-readiness", version, about = "Verify language tooling before an agent edits a repository", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Probe tools in a locked-down container made from a pinned image
    Check {
        /// Repository root to inspect
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Digest-pinned development image containing the repository tools
        #[arg(long, env = "LSP_READINESS_IMAGE", value_name = "IMAGE@sha256:DIGEST")]
        image: String,
        /// Container runtime command
        #[arg(long, default_value = "docker")]
        runtime: String,
        /// Readiness report output path
        #[arg(short, long, default_value = ".lsp-readiness.json")]
        output: PathBuf,
        /// Ed25519 private key path; created on first use
        #[arg(long, default_value = ".lsp-readiness/signing.key")]
        key: PathBuf,
        /// Skip the test command and produce a non-ready inventory
        #[arg(long)]
        skip_tests: bool,
        /// Print only the signed JSON readiness report
        #[arg(long)]
        json: bool,
    },
    /// Compatibility alias for `check`
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
        /// Readiness report output path on the host
        #[arg(short, long, default_value = ".lsp-readiness.json")]
        output: PathBuf,
        /// Ed25519 private key path on the host; created on first use
        #[arg(long, default_value = ".lsp-readiness/signing.key")]
        key: PathBuf,
        /// Skip the test command and produce a non-ready inventory
        #[arg(long)]
        skip_tests: bool,
        /// Print only the signed JSON readiness report
        #[arg(long)]
        json: bool,
    },
    /// Internal probe entry point used only inside the locked-down container
    #[command(name = "__probe", hide = true)]
    Probe {
        /// Copied repository root inside the disposable container
        path: PathBuf,
        /// Skip the test command and produce a non-ready inventory
        #[arg(long)]
        skip_tests: bool,
    },
    /// Run a deterministic probe against bundled sample repository data
    Demo {
        /// Write the sample readiness report into a new temporary directory
        #[arg(long)]
        json: bool,
    },
    /// Verify an Ed25519 signature in a readiness report
    Verify {
        /// Signed readiness report
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
            image,
            runtime,
            output,
            key,
            skip_tests,
            json,
        } => run_container(&path, &image, &runtime, &output, &key, skip_tests, json),
        Commands::Demo { json } => {
            let dir =
                std::env::temp_dir().join(format!("lsp-readiness-demo-{}", std::process::id()));
            fs::create_dir_all(&dir)?;
            let key = load_or_create_signing_key(&dir.join("signing.key"))?;
            let packet = sign(inspect_demo_fixture()?, &key)?;
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
                println!("Signed readiness report: {}", output.display());
            }
            Ok(ExitCode::SUCCESS)
        }
        Commands::Container {
            path,
            image,
            runtime,
            output,
            key,
            skip_tests,
            json,
        } => run_container(&path, &image, &runtime, &output, &key, skip_tests, json),
        Commands::Probe { path, skip_tests } => {
            if std::env::var_os("LSP_READINESS_SANDBOX").as_deref()
                != Some(std::ffi::OsStr::new("1"))
                || path != Path::new("/workspace")
            {
                anyhow::bail!("the internal probe can run only inside the readiness container");
            }
            let payload = inspect_repository(&path, !skip_tests)?;
            let ready = payload.ready;
            println!("{}", serde_json::to_string(&payload)?);
            Ok(if ready {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            })
        }
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
    skip_tests: bool,
    json: bool,
) -> Result<ExitCode> {
    validate_pinned_image(image)?;
    let repository = path
        .canonicalize()
        .with_context(|| format!("cannot open {}", path.display()))?;
    if !repository.is_dir() {
        anyhow::bail!("repository path must be a directory");
    }
    let executable = std::env::current_exe()?.canonicalize()?;

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
        "-e",
        "LSP_READINESS_SANDBOX=1",
        "-v",
    ]);
    command.arg(format!("{}:/source:ro", repository.display()));
    command.arg("-v");
    command.arg(format!(
        "{}:/usr/local/bin/lsp-readiness:ro",
        executable.display()
    ));
    command.args([image, "/bin/sh", "-c"]);
    command.arg(if skip_tests {
        "cp -R /source/. /workspace && exec /usr/local/bin/lsp-readiness __probe /workspace --skip-tests"
    } else {
        "cp -R /source/. /workspace && exec /usr/local/bin/lsp-readiness __probe /workspace"
    });
    let result = command.output().with_context(|| {
        format!("cannot start {runtime}; install it or choose --runtime podman")
    })?;
    let container_code = result.status.code().unwrap_or(2);
    if !matches!(container_code, 0 | 1) {
        let detail = String::from_utf8_lossy(&result.stderr);
        let detail = detail.trim();
        anyhow::bail!(
            "container probe failed{}",
            if detail.is_empty() {
                format!(" with exit code {container_code}")
            } else {
                format!(": {detail}")
            }
        );
    }
    let payload: ProbePayload = serde_json::from_slice(&result.stdout)
        .context("container did not return a valid readiness payload")?;
    let signing_key = load_or_create_signing_key(key)?;
    let packet = sign(payload, &signing_key)?;
    let encoded = serde_json::to_string_pretty(&packet)?;
    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, format!("{encoded}\n"))
        .with_context(|| format!("cannot write {}", output.display()))?;
    if json {
        println!("{encoded}");
    } else {
        print_report(&packet);
        println!("\nIsolated probe: {runtime} / network disabled / read-only source");
        println!("Signed readiness report: {}", output.display());
    }
    Ok(if container_code == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    })
}

fn validate_pinned_image(image: &str) -> Result<()> {
    let Some((name, digest)) = image.rsplit_once("@sha256:") else {
        anyhow::bail!(
            "container image must use an immutable sha256 digest, for example ghcr.io/team/dev@sha256:<64-hex-digest>"
        );
    };
    if name.is_empty()
        || digest.len() != 64
        || !digest
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        anyhow::bail!("container image must use a 64-character sha256 digest");
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::validate_pinned_image;

    #[test]
    fn container_image_requires_an_immutable_sha256_digest() {
        assert!(validate_pinned_image("ubuntu:latest").is_err());
        assert!(validate_pinned_image("ghcr.io/acme/dev@sha256:not-a-digest").is_err());
        assert!(validate_pinned_image(
            "ghcr.io/acme/dev@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        )
        .is_ok());
    }
}
