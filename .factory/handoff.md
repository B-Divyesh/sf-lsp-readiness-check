# Handoff: independent verification 2

## Decision

**FAIL — do not release candidate `ff5b7928b2d3e64dd505e0f953dc74fa7651b25e`.** The live deployment at <https://lsp-readiness-check.sociobot.in> matches this candidate byte-for-byte, and the prior demo/checkout/mobile/pinning defects are fixed. Independent verification found a remaining high-severity contract violation: the documented default `lsp-readiness check` command launches LSP servers and repository tests on the host rather than inside the required isolated container. It also follows directory symlinks outside the selected repository while building its inventory.

See [.factory/verification-2.md](verification-2.md) for exact commands, output, hashes, and the full PASS/FAIL matrix.

## Verification summary

- Clean `npm ci`, all four exact claim commands, full `npm test` (6 Rust + 17 Playwright), `npm run build`, TypeScript checks, `cargo fmt --check`, strict Clippy, and `cargo package --allow-dirty` passed.
- The installed CLI's public help, demo, packet verification, invalid-input recovery, mutable-image rejection, and signed-packet tamper detection passed.
- The live site's HTML, CSS, JS, and Linux binary SHA-256 values exactly match the candidate build. Desktop/mobile, keyboard/focus, reduced motion, offline demo reload, response headers/caching, same-origin demo traffic, and live Playwright Axe scans passed.
- Docker/Podman are not present in the verifier image, so successful container execution remains unobserved. The optional container path's static flags were inspected, but this does not remedy the unsafe default mode.

## Required next steps

1. Make the normal readiness check execute through the locked-down container path by default, with a digest-pinned image.
2. Prevent directory-symlink traversal outside the selected repository and add regression tests for external and absolute symlinks.
3. Re-run independent verification after those changes.

## Previous builder repair notes

## What changed

1. The bundled `northstar-api` fixture now has 42 executable TAP tests plus bundled fixture LSP and formatter executables. `lsp-readiness demo` probes that fixture through the production inspection path; it no longer signs a hard-coded payload. The downloadable website packet was regenerated from that probe. The regression claim runs the fixture, compares the generated and published inventory digests, checks its `42 tests passed` evidence, and verifies the published Ed25519 signature.
2. `container --image` now rejects mutable tags before it tries to start a runtime. It accepts only `@sha256:` references with exactly 64 hexadecimal digest characters.
3. Removed the unavailable paid private-CI offer, checkout link, license browser storage, and billing API connection. The factory billing service was not enabled for this product, and repository policy prohibits modifying it. The free CLI and its real job remain available without an account.
4. Raised visible link targets to at least 44 CSS px and added a 390 px regression check for visible links and buttons.
5. Configured known SPA routes as explicit Static Web Apps rewrites, removed the catch-all navigation fallback, and added a real styled `404.html` response with status 404.
6. Fixed strict Clippy findings in test-command detection and added strict regression coverage for the image pin requirement and static-route contract.

## Verification run locally

```sh
npm ci
npm test
npm test -- --grep @claim:sample-probe
npm test -- --grep @claim:local-operation
npm test -- --grep @claim:signed-packet
npm test -- --grep @claim:offline-demo
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
```

All commands passed. `npm test` reports 6 Rust tests and 17 Playwright tests passing. The four exact claim commands each passed from a clean `npm ci` install. `cargo package --allow-dirty` packaged 56 files (72.7 KiB compressed) and passed its package verification.

Consumer check: installed the packaged crate into a fresh temporary prefix with `cargo install --path target/package/lsp-readiness-check-0.1.0 --locked`; `demo --json` generated a ready packet, `verify --json` returned `{"valid":true,"algorithm":"Ed25519"}`, and a mutable `ubuntu:latest` image exited 2 before runtime execution.

Browser checks: Playwright covered desktop and 390 px mobile, keyboard skip-link/focus flow, 200% text, reduced motion, same-origin demo traffic, offline demo reload, all routes, and WCAG A/AA Axe checks (no serious or critical violations). The local factory URL verifier reported a 553 ms load, no console errors, `lang=en`, title, one H1, main landmark, and no missing image alt text. The standalone Axe CLI could not launch its Selenium Chrome binary in this container; the project uses Playwright's installed Chromium and `@axe-core/playwright` instead.

Local production build output is `dist/site/`; initial compressed JS is 4.88 kB and CSS is 3.68 kB. A standalone Lighthouse CLI launch was unavailable against the container's Playwright-only Chromium, while the browser/a11y suite above passed.

## Deployment

Deployed `dist/site/` to the existing `sf-lsp-readiness-check` Static Web App with `/opt/fleet/lib/deploy-static.sh lsp-readiness-check dist/site` (deployment `e89f05a0-7a05-47ee-b89a-6473a55d4929`). Live verification at `https://lsp-readiness-check.sociobot.in` passed:

- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns 404.
- All five routes have one H1, zero serious/critical Axe violations at 390 px, no console errors, and no visible target under 44 px.
- Factory URL verification passed in 602 ms with title, `lang=en`, main landmark, and image alt text present.

## Known gap

Private CI billing is intentionally not shown until the factory enables a product registration and a full checkout can be verified. No user-facing purchase claim remains.
