# Handoff: venture plan audit

## Result

No product code, deployment, credential, billing, or infrastructure setting was changed. This work added the current venture plan in [plan.md](plan.md) and the factory evidence record at `/work/.evidence/venture-plan.json`.

The current product is a functional free Rust CLI and static demo, but **M1 is not release-accepted**. The latest source-of-truth review, [review-2.md](review-2.md), is FAIL with three pending repairs. Earlier functional verification passes remain useful evidence, but do not override that later verdict.

## What was checked on 2026-09-05

```sh
npm ci
npm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
target/release/lsp-readiness demo --json
target/release/lsp-readiness verify /tmp/lsp-readiness-plan-demo.json --json
PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test
/opt/fleet/lib/verify-url.sh https://lsp-readiness-check.sociobot.in <temporary-evidence-directory>
```

- `npm test` passed: 11 Rust tests and 24 Playwright tests; `dist/site/` was built.
- Formatting, strict Clippy, and package verification passed. `cargo package` verified 58 files (79.0 KiB compressed).
- The bundled CLI demo produced a ready Ed25519 packet with 42-test evidence; `verify --json` returned valid.
- The live Playwright suite passed 24/24. `verify-url.sh` returned HTTP 200, no console errors, `lang=en`, one H1, a main landmark, no missing image alt text, and no unlabeled buttons (572 ms in this run).
- Direct live HTTP checks returned 200 for `/`, `/demo`, `/privacy`, and `/terms`; the unknown route returned 404. Security headers include a response CSP, HSTS, `nosniff`, strict referrer policy, and permissions policy.

## Remaining work before M1 can pass

1. Repair F-2-1: direct GETs of `/demo`, `/privacy`, and `/terms` currently expose the home canonical/Open Graph/Twitter metadata. Serve correct route-specific metadata before JavaScript and add raw-HTML plus hydrated-route tests.
2. Repair F-2-2: explain the signed JSON report and tamper-evident signature in plain language before using “Ed25519” and “capability packet.”
3. Repair F-2-3: explain the exact selected container image in plain language before using “digest-pinned development image.”
4. Run the normal CLI check with a real Docker or Podman engine and a digest-pinned test image. This worker has neither runtime, so existing fake-runtime tests prove the isolation argument contract but not a real engine execution.
5. Rerun the nine commands listed in [claims.json](claims.json), all quality gates, live browser/accessibility checks, and an independent adversarial review. Only then update M1 to accepted and start M2.

## Next milestone and boundaries

The next milestone is **M1 repair and operational verification**, not accounts or billing. There is currently no sign-in, GitHub App, API, SQLite persistence, messaging, billing, or paid offer. Those are explicitly planned M2/M3 dependencies in [plan.md](plan.md), not shipped capabilities.
