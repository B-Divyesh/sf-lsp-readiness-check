# Handoff: release-blocker repair

## Decision

**Ready to release.** This repair replaces the failed candidate `c1437991b5b3529925b23e54a06f70ae389ec01e`.

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

Deploy the committed build with:

```sh
/opt/fleet/lib/deploy-static.sh lsp-readiness-check dist/site
```

Then check `/`, `/demo`, `/privacy`, `/terms`, and an unknown path over the custom domain. The unknown path must return HTTP 404.

## Known gap

Private CI billing is intentionally not shown until the factory enables a product registration and a full checkout can be verified. No user-facing purchase claim remains.
