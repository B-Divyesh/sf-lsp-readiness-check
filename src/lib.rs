use anyhow::{Context, Result, anyhow, bail};
use base64::{Engine, engine::general_purpose::STANDARD as B64};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env, fs,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

pub const SCHEMA: &str = "https://lsp-readiness-check.sociobot.in/schema/v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capability {
    pub kind: String,
    pub name: String,
    pub command: String,
    pub status: CheckStatus,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Ready,
    Missing,
    Declared,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProbePayload {
    pub schema: String,
    pub repository: String,
    pub generated_at: u64,
    pub ready: bool,
    pub languages: Vec<String>,
    pub capabilities: Vec<Capability>,
    pub source_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPacket {
    pub payload: ProbePayload,
    pub algorithm: String,
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Clone)]
struct LanguageSpec {
    name: &'static str,
    extensions: &'static [&'static str],
    server: &'static str,
    server_args: &'static [&'static str],
    formatter: &'static str,
}

const LANGUAGE_SPECS: &[LanguageSpec] = &[
    LanguageSpec {
        name: "JavaScript / TypeScript",
        extensions: &["js", "jsx", "ts", "tsx", "mjs", "cjs"],
        server: "typescript-language-server",
        server_args: &["--stdio"],
        formatter: "prettier",
    },
    LanguageSpec {
        name: "Rust",
        extensions: &["rs"],
        server: "rust-analyzer",
        server_args: &[],
        formatter: "rustfmt",
    },
    LanguageSpec {
        name: "Python",
        extensions: &["py"],
        server: "pyright-langserver",
        server_args: &["--stdio"],
        formatter: "ruff",
    },
    LanguageSpec {
        name: "Go",
        extensions: &["go"],
        server: "gopls",
        server_args: &["serve"],
        formatter: "gofmt",
    },
    LanguageSpec {
        name: "Svelte",
        extensions: &["svelte"],
        server: "svelteserver",
        server_args: &["--stdio"],
        formatter: "prettier",
    },
];

pub fn inspect_repository(path: &Path, run_tests: bool) -> Result<ProbePayload> {
    let root = path
        .canonicalize()
        .with_context(|| format!("cannot open {}", path.display()))?;
    let files = walk_source_files(&root)?;
    if files.is_empty() {
        bail!("no source or package files found; point the command at a repository root");
    }

    let mut languages = Vec::new();
    let mut capabilities = Vec::new();
    for spec in LANGUAGE_SPECS {
        if files.iter().any(|p| {
            p.extension()
                .and_then(|v| v.to_str())
                .is_some_and(|ext| spec.extensions.contains(&ext))
        }) {
            languages.push(spec.name.to_string());
            capabilities.push(probe_lsp(&root, spec));
            capabilities.push(probe_executable("formatter", spec.formatter));
        }
    }

    let test = detect_test_command(&root);
    capabilities.push(match test {
        Some(command) if run_tests => run_test_command(&root, &command),
        Some(command) => Capability {
            kind: "tests".into(),
            name: "Repository tests".into(),
            command,
            status: CheckStatus::Declared,
            evidence: "declared; pass --run-tests to execute".into(),
        },
        None => Capability {
            kind: "tests".into(),
            name: "Repository tests".into(),
            command: "none detected".into(),
            status: CheckStatus::Missing,
            evidence: "no supported test manifest found".into(),
        },
    });

    let ready = !languages.is_empty()
        && capabilities
            .iter()
            .filter(|c| c.kind == "lsp")
            .all(|c| c.status == CheckStatus::Ready)
        && capabilities.iter().any(|c| {
            c.kind == "tests" && matches!(c.status, CheckStatus::Ready | CheckStatus::Declared)
        });

    Ok(ProbePayload {
        schema: SCHEMA.into(),
        repository: root
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("repository")
            .into(),
        generated_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        ready,
        languages,
        capabilities,
        source_digest: digest_inventory(&root, &files)?,
    })
}

pub fn demo_payload() -> ProbePayload {
    ProbePayload {
        schema: SCHEMA.into(),
        repository: "northstar-api".into(),
        generated_at: 1_788_307_200,
        ready: true,
        languages: vec!["JavaScript / TypeScript".into(), "Rust".into()],
        capabilities: vec![
            Capability {
                kind: "lsp".into(),
                name: "JavaScript / TypeScript language server".into(),
                command: "typescript-language-server --stdio".into(),
                status: CheckStatus::Ready,
                evidence:
                    "initialize reply received in 184 ms; definition, references, diagnostics"
                        .into(),
            },
            Capability {
                kind: "formatter".into(),
                name: "prettier".into(),
                command: "prettier".into(),
                status: CheckStatus::Ready,
                evidence: "prettier 3.6.2".into(),
            },
            Capability {
                kind: "lsp".into(),
                name: "Rust language server".into(),
                command: "rust-analyzer".into(),
                status: CheckStatus::Ready,
                evidence:
                    "initialize reply received in 231 ms; definition, references, diagnostics"
                        .into(),
            },
            Capability {
                kind: "formatter".into(),
                name: "rustfmt".into(),
                command: "rustfmt".into(),
                status: CheckStatus::Ready,
                evidence: "rustfmt 1.8.0-stable".into(),
            },
            Capability {
                kind: "tests".into(),
                name: "Repository tests".into(),
                command: "npm test".into(),
                status: CheckStatus::Ready,
                evidence: "42 tests passed".into(),
            },
        ],
        source_digest: "sha256:1df019b6566159a4b012f1d68133f30aa80b16ec81ce1785a93a555a07b93e2d"
            .into(),
    }
}

pub fn load_or_create_signing_key(path: &Path) -> Result<SigningKey> {
    if path.exists() {
        let bytes = fs::read(path)
            .with_context(|| format!("cannot read signing key {}", path.display()))?;
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| anyhow!("signing key must contain exactly 32 bytes"))?;
        return Ok(SigningKey::from_bytes(&bytes));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let key = SigningKey::generate(&mut OsRng);
    fs::write(path, key.to_bytes())
        .with_context(|| format!("cannot write signing key {}", path.display()))?;
    set_owner_only(path)?;
    Ok(key)
}

#[cfg(unix)]
fn set_owner_only(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_owner_only(_path: &Path) -> Result<()> {
    Ok(())
}

pub fn sign(payload: ProbePayload, key: &SigningKey) -> Result<SignedPacket> {
    let message = serde_json::to_vec(&payload)?;
    Ok(SignedPacket {
        payload,
        algorithm: "Ed25519".into(),
        public_key: B64.encode(key.verifying_key().to_bytes()),
        signature: B64.encode(key.sign(&message).to_bytes()),
    })
}

pub fn verify(packet: &SignedPacket) -> Result<()> {
    if packet.algorithm != "Ed25519" {
        bail!("unsupported signature algorithm");
    }
    let public: [u8; 32] = B64
        .decode(&packet.public_key)?
        .try_into()
        .map_err(|_| anyhow!("invalid public key length"))?;
    let signature: [u8; 64] = B64
        .decode(&packet.signature)?
        .try_into()
        .map_err(|_| anyhow!("invalid signature length"))?;
    VerifyingKey::from_bytes(&public)?
        .verify(
            &serde_json::to_vec(&packet.payload)?,
            &Signature::from_bytes(&signature),
        )
        .map_err(|_| anyhow!("signature does not match the capability packet"))
}

fn walk_source_files(root: &Path) -> Result<Vec<PathBuf>> {
    fn visit(dir: &Path, root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let rel = path.strip_prefix(root).unwrap_or(&path);
            if path.is_dir() {
                let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("");
                if matches!(
                    name,
                    ".git" | "node_modules" | "target" | "dist" | "vendor" | ".venv"
                ) {
                    continue;
                }
                visit(&path, root, out)?;
            } else if is_relevant(rel) {
                out.push(path);
                if out.len() > 10_000 {
                    bail!("repository scan stopped at 10,000 relevant files");
                }
            }
        }
        Ok(())
    }
    let mut out = Vec::new();
    visit(root, root, &mut out)?;
    out.sort();
    Ok(out)
}

fn is_relevant(path: &Path) -> bool {
    let name = path.file_name().and_then(|v| v.to_str()).unwrap_or("");
    matches!(
        name,
        "package.json" | "Cargo.toml" | "pyproject.toml" | "go.mod"
    ) || path
        .extension()
        .and_then(|v| v.to_str())
        .is_some_and(|ext| LANGUAGE_SPECS.iter().any(|s| s.extensions.contains(&ext)))
}

fn executable_path(command: &str) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .map(|p| p.join(command))
            .find(|p| p.is_file())
    })
}

fn probe_executable(kind: &str, command: &str) -> Capability {
    let Some(path) = executable_path(command) else {
        return Capability {
            kind: kind.into(),
            name: command.into(),
            command: command.into(),
            status: CheckStatus::Missing,
            evidence: "command not found on PATH".into(),
        };
    };
    let output = Command::new(&path).arg("--version").output();
    match output {
        Ok(output) if output.status.success() => {
            let evidence = String::from_utf8_lossy(if output.stdout.is_empty() {
                &output.stderr
            } else {
                &output.stdout
            })
            .trim()
            .lines()
            .next()
            .unwrap_or("version command passed")
            .to_string();
            Capability {
                kind: kind.into(),
                name: command.into(),
                command: command.into(),
                status: CheckStatus::Ready,
                evidence,
            }
        }
        Ok(output) => Capability {
            kind: kind.into(),
            name: command.into(),
            command: command.into(),
            status: CheckStatus::Failed,
            evidence: format!("version command exited with {}", output.status),
        },
        Err(error) => Capability {
            kind: kind.into(),
            name: command.into(),
            command: command.into(),
            status: CheckStatus::Failed,
            evidence: error.to_string(),
        },
    }
}

fn probe_lsp(root: &Path, spec: &LanguageSpec) -> Capability {
    let command_label = std::iter::once(spec.server)
        .chain(spec.server_args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ");
    let Some(path) = executable_path(spec.server) else {
        return Capability {
            kind: "lsp".into(),
            name: format!("{} language server", spec.name),
            command: command_label,
            status: CheckStatus::Missing,
            evidence: "command not found on PATH".into(),
        };
    };
    let mut child = match Command::new(path)
        .args(spec.server_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .current_dir(root)
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return Capability {
                kind: "lsp".into(),
                name: format!("{} language server", spec.name),
                command: command_label,
                status: CheckStatus::Failed,
                evidence: error.to_string(),
            };
        }
    };
    let request = serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"rootUri":format!("file://{}", root.display()),"capabilities":{}}}).to_string();
    if child.stdin.as_mut().is_none_or(|stdin| {
        write!(
            stdin,
            "Content-Length: {}\r\n\r\n{}",
            request.len(),
            request
        )
        .is_err()
    }) {
        let _ = child.kill();
        return Capability {
            kind: "lsp".into(),
            name: format!("{} language server", spec.name),
            command: command_label,
            status: CheckStatus::Failed,
            evidence: "could not send initialize request".into(),
        };
    }
    drop(child.stdin.take());
    let stdout = child.stdout.take().expect("piped stdout");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(read_lsp_reply(stdout));
    });
    let result = rx.recv_timeout(Duration::from_secs(5));
    let _ = child.kill();
    let _ = child.wait();
    match result {
        Ok(Ok(reply)) if reply.get("result").is_some() => {
            let (status, evidence) = summarize_lsp_capabilities(&reply);
            Capability {
                kind: "lsp".into(),
                name: format!("{} language server", spec.name),
                command: command_label,
                status,
                evidence,
            }
        }
        Ok(Ok(_)) => Capability {
            kind: "lsp".into(),
            name: format!("{} language server", spec.name),
            command: command_label,
            status: CheckStatus::Failed,
            evidence: "initialize reply did not contain capabilities".into(),
        },
        Ok(Err(error)) => Capability {
            kind: "lsp".into(),
            name: format!("{} language server", spec.name),
            command: command_label,
            status: CheckStatus::Failed,
            evidence: error.to_string(),
        },
        Err(_) => Capability {
            kind: "lsp".into(),
            name: format!("{} language server", spec.name),
            command: command_label,
            status: CheckStatus::Failed,
            evidence: "initialize timed out after 5 seconds".into(),
        },
    }
}

fn summarize_lsp_capabilities(reply: &serde_json::Value) -> (CheckStatus, String) {
    let capabilities = reply.pointer("/result/capabilities");
    let enabled = |name: &str| {
        capabilities
            .and_then(|value| value.get(name))
            .is_some_and(|value| value.as_bool().unwrap_or(!value.is_null()))
    };
    let definition = enabled("definitionProvider");
    let references = enabled("referencesProvider");
    let diagnostics = enabled("diagnosticProvider") || enabled("textDocumentSync");
    let mut found = Vec::new();
    if definition {
        found.push("definition");
    }
    if references {
        found.push("references");
    }
    if diagnostics {
        found.push("diagnostics");
    }
    let status = if definition && references && diagnostics {
        CheckStatus::Ready
    } else {
        CheckStatus::Failed
    };
    let evidence = if found.is_empty() {
        "initialize reply had none of the required capabilities".into()
    } else {
        format!("initialize reply; capabilities: {}", found.join(", "))
    };
    (status, evidence)
}

fn read_lsp_reply(reader: impl Read) -> Result<serde_json::Value> {
    let mut reader = BufReader::new(reader);
    let mut length = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            bail!("language server closed before replying");
        }
        if line == "\r\n" {
            break;
        }
        if let Some(value) = line.strip_prefix("Content-Length:") {
            length = Some(value.trim().parse::<usize>()?);
        }
    }
    let mut body =
        vec![0; length.ok_or_else(|| anyhow!("language server reply had no content length"))?];
    reader.read_exact(&mut body)?;
    Ok(serde_json::from_slice(&body)?)
}

fn detect_test_command(root: &Path) -> Option<String> {
    if root.join("package.json").exists() {
        if let Ok(value) =
            serde_json::from_slice::<serde_json::Value>(&fs::read(root.join("package.json")).ok()?)
        {
            if value
                .pointer("/scripts/test")
                .and_then(|v| v.as_str())
                .is_some_and(|v| !v.contains("no test specified"))
            {
                return Some("npm test".into());
            }
        }
    }
    if root.join("Cargo.toml").exists() {
        return Some("cargo test".into());
    }
    if root.join("pyproject.toml").exists() {
        return Some("python -m pytest".into());
    }
    if root.join("go.mod").exists() {
        return Some("go test ./...".into());
    }
    None
}

fn run_test_command(root: &Path, command: &str) -> Capability {
    let mut parts = command.split_whitespace();
    let binary = parts.next().unwrap_or(command);
    match Command::new(binary).args(parts).current_dir(root).status() {
        Ok(status) if status.success() => Capability {
            kind: "tests".into(),
            name: "Repository tests".into(),
            command: command.into(),
            status: CheckStatus::Ready,
            evidence: "test command passed".into(),
        },
        Ok(status) => Capability {
            kind: "tests".into(),
            name: "Repository tests".into(),
            command: command.into(),
            status: CheckStatus::Failed,
            evidence: format!("test command exited with {}", status),
        },
        Err(error) => Capability {
            kind: "tests".into(),
            name: "Repository tests".into(),
            command: command.into(),
            status: CheckStatus::Failed,
            evidence: error.to_string(),
        },
    }
}

fn digest_inventory(root: &Path, files: &[PathBuf]) -> Result<String> {
    let mut hasher = Sha256::new();
    for file in files {
        let metadata = fs::metadata(file)?;
        hasher.update(file.strip_prefix(root)?.to_string_lossy().as_bytes());
        hasher.update(metadata.len().to_le_bytes());
    }
    Ok(format!("sha256:{:x}", hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_signature_verifies_and_detects_changes() {
        let key = SigningKey::generate(&mut OsRng);
        let mut packet = sign(demo_payload(), &key).unwrap();
        verify(&packet).unwrap();
        packet.payload.repository = "changed".into();
        assert!(verify(&packet).is_err());
    }

    #[test]
    fn empty_repository_has_a_plain_error() {
        let dir = tempfile::tempdir().unwrap();
        let error = inspect_repository(dir.path(), false)
            .unwrap_err()
            .to_string();
        assert!(error.contains("no source or package files found"));
    }

    #[test]
    fn demo_has_each_required_capability() {
        let packet = demo_payload();
        assert!(packet.ready);
        for kind in ["lsp", "formatter", "tests"] {
            assert!(packet.capabilities.iter().any(|cap| cap.kind == kind));
        }
    }

    #[test]
    fn lsp_capability_summary_requires_navigation_and_diagnostics() {
        let complete = serde_json::json!({"result":{"capabilities":{"definitionProvider":true,"referencesProvider":true,"textDocumentSync":2}}});
        let partial = serde_json::json!({"result":{"capabilities":{"definitionProvider":true}}});
        assert_eq!(summarize_lsp_capabilities(&complete).0, CheckStatus::Ready);
        assert_eq!(summarize_lsp_capabilities(&partial).0, CheckStatus::Failed);
    }
}
