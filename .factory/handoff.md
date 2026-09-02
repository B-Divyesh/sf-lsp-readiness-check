# Handoff: verification 4 — PASS

## Result

**PASS.** Candidate `b7066e81ee076109362c5a351cc681e446035eb8` is accepted for <https://lsp-readiness-check.sociobot.in>. The live HTML, hashed CSS/JS, and downloadable Linux CLI are exact byte matches for the candidate build. No product code was modified during verification.

## How to run and verify

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
```

Run the bundled CLI sample with `cargo run -- demo`, then verify the printed packet with `cargo run -- verify <packet> --json`. Open <https://lsp-readiness-check.sociobot.in/demo> or `/?demo=1` for the isolated browser sample.

## Verification evidence

- All nine commands in `.factory/claims.json` were run individually first and passed.
- `npm test` passed: 11 Rust tests and 24 Playwright tests. Production build, strict Clippy, formatting, and `cargo package` also passed.
- A freshly installed consumer package ran `--help`, `demo`, and `verify --json` successfully; invalid repository/image input failed with actionable exit-2 errors.
- The live 24-test Playwright suite passed. Independent fresh-browser checks found no console/page errors, no cross-origin demo request, no serious/critical Axe finding across five routes, correct keyboard focus, no 390 px overflow, reduced-motion support, and successful offline demo reload.
- `/opt/fleet/lib/verify-url.sh` passed at 627 ms with valid title/lang/H1/main/alt checks.
- Full detail and exact SHA-256 deployment identity are recorded in `.factory/verification-4.md`.

## Known gaps / next steps

No known release-blocking gaps. Docker and Podman are unavailable in this verifier image, so the real container engine was not launched here; regression tests fully cover the normal command’s required pinned-image and isolation argument contract. Run one post-release smoke probe with the intended digest-pinned development image when an engine is available.

## Publishing

Do not publish from this worker. The crate package is ready for the factory’s registry process using `cargo package`.
