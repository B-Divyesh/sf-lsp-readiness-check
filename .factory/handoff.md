# Handoff: M1 acceptance repair

## Status

**PASS — M1 remains accepted after repairing verification-5.** The free local CLI and static demo now use a shared, plain-language 404 and consistently call their signed output a **readiness report**. There are zero untested public claims.

| Record | SHA / identifier |
| --- | --- |
| Implementation | `748178140e4f46e75bc596086f09da9bfd3605ba` |
| Documentation deployed with it | `01102b7be63059becb95b13f47222ebfc274270a` |
| Static Web App deployment | `fb33c0f7-9af2-428f-969a-8a41f8f7373e` |

The implementation and documentation commits are intentionally separate. This handoff is a later report-only update; it does not require a new product deployment.

## What changed

- Replaced the static-host 404 document with the shared skip link, wordmark header, primary navigation, main landmark, footer, page-specific metadata, plain **Page not found** heading, and return-home action. Unknown URLs still return HTTP 404.
- Replaced visitor-facing “packet” wording with **readiness report** in the demo, privacy page, README, demo guide, claims manifest, CLI help, normal CLI completion output, and signature error. Internal `SignedPacket` names and the stable `signed-packet` claim id remain technical implementation details.
- Added browser regressions that prove an unknown static URL has the shared structure and takes a visitor home, and that the demo/privacy surfaces show the agreed output term. These assert rendered visitor outcomes, not source-file strings.
- Added the README deployment note and copied the verb-first catalog description unchanged to `/work/.evidence/catalog-description.txt`.

## Verification

From a clean clone of documentation SHA `01102b7be63059becb95b13f47222ebfc274270a`, `npm ci` completed and every exact command in `.factory/claims.json` passed:

```sh
npm test -- --grep @claim:sample-probe
npm test -- --grep @claim:local-operation
npm test -- --grep @claim:signed-packet
npm test -- --grep @claim:offline-demo
npm test -- --grep @claim:no-account
npm test -- --grep @claim:no-tool-install
npm test -- --grep @claim:no-dependency-install
npm test -- --grep @claim:noninteractive-ci
npm test -- --grep @claim:signing-key-permissions
```

The local full suite passed: **11 Rust tests and 27 Playwright tests**. `npm run build`, TypeScript checks, `cargo fmt --check`, strict Clippy, and `cargo package --allow-dirty` passed. The package contained 58 files (79.9 KiB compressed) and installed into a fresh consumer prefix. The installed CLI exposed helpful help, printed `Signed readiness report`, verified its demo JSON, and rejected a mutable image before runtime startup.

Live checks after deployment:

- `/opt/fleet/lib/verify-url.sh` passed: HTTPS 200, 687 ms load, no console errors, `lang=en`, one H1, main landmark, alt coverage, and named buttons.
- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` passed **27/27**, including Axe serious/critical checks, keyboard/focus, 390 px targets, reduced motion, same-origin traffic, offline demo reload, routes, 404, and links.
- Fresh 1366×900 desktop and 390×844 phone contexts started at `scrollY = 0`. Both showed the job, audience, and **Try it with sample data** action inside the viewport; phone `scrollWidth` was 390.
- A fresh live demo context seeded `real:lsp-readiness-check=sentinel`. Running and resetting demo kept that value; **Start for real** removed only `demo:lsp-readiness-check`. Requests stayed same-origin.
- Root returned 200; an intentional unknown route returned 404 with a header, named navigation, footer, and **Page not found** H1.
- Lighthouse mobile reported performance **99**, accessibility **100**, best practices **100**, and SEO **100**; FCP 0.9 s, LCP 1.8 s, TBT 110 ms, CLS 0. Initial JavaScript is 5.12 KiB gzip and CSS is 3.68 KiB gzip.

## Deployment identity

The live artifacts exactly match the clean local build:

| Artifact | SHA-256 |
| --- | --- |
| Root HTML | `b696919082e4626639113ce651b5d9719252c243baa2a4860b01dd87b276014d` |
| Static 404 HTML | `f87d4e8ad1057d5b63d61b935ca3405755a4b2fae2dd3dfb58331ec122c559cb` |
| JavaScript | `f2bed6f847af466fb0093baee56fa760f99544cd442ae595ed70fc678d7be44f` |
| CSS | `d9ab7665da6abd91151642632867314f15636b53805bc7b2f738fdeba2ff5639` |
| Linux CLI | `4e7cc788275bd352a5c91b60ca7606d9b9863a0f33d3205c5cc15b4b03dd9fbd` |

The downloaded live CLI ran `demo --json` and `verify --json` successfully and reports version 0.1.1.

## Docker evidence and deployment shape

The prior product-scoped Docker matrix remains part of the M1 evidence. At implementation `b3714c16ec78b14d5d403d7eaa98e5ac0b27ee02`, a real Docker engine ran the normal command against controlled digest-pinned Ubuntu 24.04 images: ready exited 0 with a verified signed result; non-ready exited 1 with a verified result; an LSP timeout exited 1 with five-second timeout evidence; and a BusyBox runtime mismatch exited 2 without changing source. Every source checksum was unchanged. The non-sensitive matrix summary is retained at `/work/.evidence/lsp-readiness-check-qa-vm-matrix-ubuntu-summary.txt` in the originating worker record.

This repair did not alter container invocation, image validation, source mounts, security flags, or signing; the only Rust changes are user-facing report terminology. Current executable isolation claims passed again. Docker is not installed in this worker, so no new real-engine run was invented or claimed. Podman remains a separate customer-environment compatibility dependency.

The assigned product is a **Static Web App**. It has no Dockerfile, Container App, ACR image, process health endpoint, SQLite database, or `/data` mount. The static deployment preserved its existing product resource and did not create a container service, so `/data` preservation and container-image digest checks are not applicable. The applicable M1 persistence boundary is browser demo storage; its separate `demo:` namespace and real-data sentinel preservation are live-verified above.

## Scope and next steps

M1 ships no account, backend API, tenant data, paid offer, checkout, GitHub App, analytics, or billing call. Tenant isolation, restart persistence, health API, 429/`Retry-After`, billing registration, and subscription entitlement are therefore not M1 checks. The researched $49/repository/month private-CI subscription remains planned for M2 and is not advertised as available.

M2 separately depends on factory-provisioned Entra CIAM, GitHub App registration, Sociobot subscription registration, a product `/data` SQLite mount, tenant isolation, and rate limits. Do not present any of those as shipped before that milestone.
