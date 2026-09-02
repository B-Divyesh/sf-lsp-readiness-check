#![cfg(unix)]

use std::{fs, os::unix::fs::PermissionsExt, process::Command};

use lsp_readiness_check::{SignedPacket, verify};

const PINNED_IMAGE: &str = "registry.example/readiness@sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn normal_check_invokes_the_locked_down_container_path() {
    let boundary = tempfile::tempdir().unwrap();
    let repository = boundary.path().join("repository");
    let runtime = boundary.path().join("capture-runtime");
    let capture = boundary.path().join("arguments.txt");
    fs::create_dir(&repository).unwrap();
    fs::write(repository.join("source.rs"), "fn main() {}").unwrap();
    fs::write(
        &runtime,
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$LSP_READINESS_CAPTURE\"\nexit 23\n",
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o700)).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_lsp-readiness"))
        .env("LSP_READINESS_CAPTURE", &capture)
        .args([
            "check",
            repository.to_str().unwrap(),
            "--image",
            PINNED_IMAGE,
            "--runtime",
            runtime.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!result.status.success());

    let arguments = fs::read_to_string(capture).unwrap();
    for required in [
        "--network\nnone\n",
        "--read-only\n",
        "--cap-drop=ALL\n",
        "--security-opt=no-new-privileges\n",
        "--tmpfs\n/workspace:rw,exec,nosuid,size=1g\n",
        "--tmpfs\n/tmp:rw,noexec,nosuid,size=256m\n",
        "LSP_READINESS_SANDBOX=1\n",
        PINNED_IMAGE,
        "lsp-readiness __probe /workspace",
    ] {
        assert!(
            arguments.contains(required),
            "missing {required:?} in {arguments}"
        );
    }
    assert!(arguments.contains(&format!(
        "{}:/source:ro",
        repository.canonicalize().unwrap().display()
    )));
    assert!(!arguments.contains("lsp-readiness check /workspace"));
}

#[test]
fn normal_check_fails_closed_without_a_pinned_image() {
    let boundary = tempfile::tempdir().unwrap();
    let repository = boundary.path().join("repository");
    fs::create_dir(&repository).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_lsp-readiness"))
        .args(["check", repository.to_str().unwrap()])
        .output()
        .unwrap();

    assert_eq!(result.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&result.stderr).contains("--image"));
}

#[test]
fn host_signs_the_payload_returned_by_the_container() {
    let boundary = tempfile::tempdir().unwrap();
    let repository = boundary.path().join("repository");
    let runtime = boundary.path().join("fixture-runtime");
    let output = boundary.path().join("packet.json");
    let key = boundary.path().join("signing.key");
    fs::create_dir(&repository).unwrap();
    let payload = serde_json::json!({
        "schema": "https://lsp-readiness-check.sociobot.in/schema/v1",
        "repository": "repository",
        "generated_at": 1,
        "ready": true,
        "languages": ["Rust"],
        "capabilities": [],
        "source_digest": "sha256:fixture"
    });
    fs::write(
        &runtime,
        format!("#!/bin/sh\nprintf '%s\\n' '{}'\n", payload),
    )
    .unwrap();
    fs::set_permissions(&runtime, fs::Permissions::from_mode(0o700)).unwrap();

    let result = Command::new(env!("CARGO_BIN_EXE_lsp-readiness"))
        .args([
            "check",
            repository.to_str().unwrap(),
            "--image",
            PINNED_IMAGE,
            "--runtime",
            runtime.to_str().unwrap(),
            "--output",
            output.to_str().unwrap(),
            "--key",
            key.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let packet: SignedPacket = serde_json::from_slice(&fs::read(output).unwrap()).unwrap();
    verify(&packet).unwrap();
    assert_eq!(packet.payload.repository, "repository");
    assert!(key.is_file());
}
