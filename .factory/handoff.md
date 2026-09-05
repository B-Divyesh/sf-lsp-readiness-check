# Handoff: M1 repair, acceptance, and verification 5

## Status

**Verification 5: FAIL with two minor findings and zero untested claims.** The deployed M1 implementation still passes its nine claims, installed CLI checks, 25-test live browser suite, accessibility checks, and performance budgets. The live 404 lacks the standard header/footer and uses metaphor copy; privacy/demo/README copy also uses the old “packet” term instead of the documented “readiness report.” See [verification-5.md](verification-5.md).

This is still a free local CLI and static demo. It has no account, server API, SQLite database, billing, GitHub App, or paid offer. M2 is planned only and is not part of this verdict.

| Record | SHA |
| --- | --- |
| Deployed implementation | `b3714c16ec78b14d5d403d7eaa98e5ac0b27ee02` |
| Documentation reviewed by verification 5 | `35b58749f91feac2bc155534be303167e6ad8fd5` |

The static deployment for the implementation used deployment `3e0c28c6-9671-4de2-84cf-e3b87c287205`. Later report-only commits do not change the deployed product image.

## What changed

- Added a shared route-metadata manifest and a production prerender step. `/`, `/demo`, `/privacy`, `/terms`, and `/404` now have their own title, description, canonical, Open Graph, and Twitter tags before JavaScript. Client-side navigation updates the same tags.
- Changed static-host route rewrites to each prerendered document. The test server now applies the same output routing as the deployed static host, rather than Vite's SPA fallback.
- Rewrote the landing and README to say **signed JSON readiness report** first, explain tamper detection, and explain why the selected image uses a SHA-256 address. Ed25519 remains an implementation detail.
- Added raw-response plus hydrated-route metadata coverage, visible plain-copy coverage, and an outcome-based locked-container regression that captures the invoked runtime arguments instead of reading source strings.
- Clarified the real-engine discovery in the README: the selected image must have a glibc version compatible with the installed CLI binary.

## Former findings

| Finding | Current disposition |
| --- | --- |
| F-2-1: routed social metadata described the home page | Fixed and live-verified with direct GETs plus raw/hydrated browser regression. |
| F-2-2: unexplained capability-packet / Ed25519 wording | Fixed; the landing leads with the user outcome and tamper-detection explanation. |
| F-2-3: unexplained digest-pinned image wording | Fixed; landing and install copy explain selecting the exact image and SHA-256 address. |
| Earlier F-1 claim, touch-target, mutable-image, default-container, symlink, paid-offer, 404, and lint findings | Remain covered by the nine current claims, Rust isolation suite, 25-test browser suite, static 404 check, and strict Clippy/package checks. No paid offer or checkout is present. |

## Real container-engine evidence

Docker/Podman were absent in the worker. A product-scoped helper, `sf-lsp-readiness-check-qa-vm`, ran the deployed implementation through Docker. It had a private NIC, an explicit deny-all inbound NSG rule, no public IP on the VM, and only a product-named NAT gateway for outbound package/image downloads. It is **deallocated**; no Azure resource was deleted.

The helper cloned implementation SHA `b3714c16ec78b14d5d403d7eaa98e5ac0b27ee02`, built the CLI, and used a localhost registry containing controlled digest-pinned test images. No production product setting, secret, database, or service was accessed.

| Case | Result | Packet/source evidence |
| --- | --- | --- |
| Ready | Exit 0 in 1 s | Signed packet verified; `ready: true`; source checksum unchanged. |
| Non-ready | Exit 1 in 2 s | Signed packet verified; missing LSP/formatter evidence; source checksum unchanged. |
| LSP timeout | Exit 1 in 8 s | Signed packet verified; `initialize timed out after 5 seconds`; source checksum unchanged. |
| Runtime error | Exit 2 in under 1 s | Actual Docker runtime failed to load the mounted binary in the BusyBox fixture; source checksum unchanged. |

The four non-sensitive image digests and complete command result are retained in `/work/.evidence/lsp-readiness-check-qa-vm-matrix-ubuntu-summary.txt`. The ready/non-ready/timeout/runtime-error images were all addressed as `localhost:5000/sf-lsp-readiness-check-*@sha256:…` during the run.

The first Bookworm image attempt exposed a glibc 2.39 mismatch before a probe began; it was not counted as a passing test. The successful Ubuntu 24.04 images match the candidate binary's glibc. This is why the README now states the compatibility condition plainly.

## Verification

From a fresh clone of the deployed implementation:

```sh
npm ci
npm test -- --grep @claim:sample-probe
npm test -- --grep @claim:local-operation
npm test -- --grep @claim:signed-packet
npm test -- --grep @claim:offline-demo
npm test -- --grep @claim:no-account
npm test -- --grep @claim:no-tool-install
npm test -- --grep @claim:no-dependency-install
npm test -- --grep @claim:noninteractive-ci
npm test -- --grep @claim:signing-key-permissions
npm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
cargo install --path target/package/lsp-readiness-check-0.1.1 --root <fresh-prefix> --locked
<fresh-prefix>/bin/lsp-readiness demo --json
<fresh-prefix>/bin/lsp-readiness verify <packet> --json
```

All nine claims passed. The full suite passed with 11 Rust tests and 25 Playwright tests. Strict formatting, Clippy, package verification, and the clean consumer install/demo/verify passed. The installed consumer also rejected mutable `ubuntu:latest` with exit 2 before it tried the named runtime.

Live verification passed:

- `verify-url.sh` reported HTTPS 200, 745 ms cold load, no console errors, `lang=en`, one H1, main, image alt coverage, and named buttons.
- Direct live GETs returned 200 for `/`, `/demo`, `/privacy`, and `/terms`, each with the correct prerendered metadata. An unknown route returned the designed HTTP 404.
- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` passed 25/25, including Axe serious/critical checks, keyboard/focus, 390 px layout, reduced motion, offline demo reload, demo reset/isolation, same-origin traffic, and raw/hydrated route metadata.
- Fresh 1366 px desktop and 390 px phone contexts showed the job (“Verify tooling before an agent edits”), audience, and “Try it with sample data” before scrolling. Neither view overflowed horizontally or logged errors.
- Downloaded live `index.html`, JavaScript, and CSS SHA-256 values matched `dist/site/` exactly.

The earlier Lighthouse attempt crashed before producing a score. Verification 5 reran Lighthouse 12.8.2 successfully: performance 99, accessibility 100, best practices 100, SEO 100, LCP 1.8 s, total blocking time 90 ms, and CLS 0. Bundle sizes remain small: initial JavaScript is 5.13 kB gzip, CSS is 3.68 kB gzip, the self-hosted font is about 67 kB, and the hero WebP is about 120 kB.

## Product-scoped helper resources

- `sf-lsp-readiness-check-qa-vm` — `Standard_B2s`, deallocated.
- `sf-lsp-readiness-check-qa-vm-osdisk`, `sf-lsp-readiness-check-qa-vm-nic`.
- `sf-lsp-readiness-check-qa-vnet`, `sf-lsp-readiness-check-qa-subnet`, `sf-lsp-readiness-check-qa-nsg` with `deny-all-inbound`.
- `sf-lsp-readiness-check-qa-nat`, `sf-lsp-readiness-check-qa-nat-pip` for outbound-only helper access.

## Remaining dependencies and next step

- Repair F-5-1 and F-5-2 in [verification-5.md](verification-5.md), deploy the corrected static output, and rerun the live 404/copy checks before returning M1 to PASS.
- Customer CI still needs Docker or Podman and a SHA-256-pinned development image containing its own tools and dependencies. Docker is now validated; Podman has not been validated on a real engine.
- M2 needs factory-provisioned Entra CIAM, a GitHub App, a Sociobot subscription contract, and a product `/data` SQLite mount before any account, private CI, history, or billing capability is built or claimed.
