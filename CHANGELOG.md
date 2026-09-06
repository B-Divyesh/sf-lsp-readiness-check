# Changelog

## 0.1.2 — 2026-09-06

- Add the M2 tenant-scoped Rust API and durable SQLite schema.
- Add CIAM PKCE, GitHub App connection, repository policy, export/delete, health, and rate-limit foundations.
- Publish the exact $49/repository/month offer as unavailable until recurring billing registration and hosted entitlement QA finish.
- Keep the accepted free CLI and isolated sample behavior unchanged.

## 0.1.1 — 2026-09-02

- Make `check` run through a digest-pinned, network-disabled container by default.
- Keep signing keys on the host and outside the repository sandbox.
- Skip relative, absolute, file, and directory symlinks during repository discovery.
- Add executable container-policy and symlink-boundary regression tests.

## 0.1.0 — 2026-09-02

- Add repository language detection and LSP initialize probes.
- Add formatter and test-command checks.
- Add Ed25519-signed JSON capability packets and verification.
- Add the bundled `northstar-api` demo.
- Add the static documentation, browser demo, and legal pages.
