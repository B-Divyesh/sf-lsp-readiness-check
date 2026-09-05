# Verification 6 — verify tooling before agent edits

- Work order: `lsp-readiness-check-verify-6`
- Current milestone: M1 — free local CLI and static demo
- Implementation reviewed: `748178140e4f46e75bc596086f09da9bfd3605ba`
- Documentation baseline reviewed: `2006c75a7c7784e982e3cf8869b1a31366eed090`
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Verified: 2026-09-05 UTC
- Verdict: **PASS**
- Findings: **0**
- Untested claims: **0**

## Decision

**PASS — zero findings of every severity and zero untested claims.** M1 works end to end as a free local readiness CLI and one-click static demo. The deployed site and Linux download match the canonical candidate build. All nine public claim commands pass from a fresh checkout, and the packaged CLI works from a separate consumer prefix.

M2 is not part of this verdict. Accounts, private CI, billing, tenant data, service persistence, health, and rate limiting are planned capabilities and are not presented as shipped.

## First screen before scrolling

Fresh Chromium contexts opened the live page at 1366×900 and 390×844. Both began at `scrollY = 0`, had no console or page errors, and showed the primary action fully inside the viewport.

| Question | Live answer |
| --- | --- |
| Job | “Verify tooling before an agent edits” |
| Audience | “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.” |
| First action | “Try it with sample data” — “See a finished probe in one click.” |

The phone layout had `scrollWidth = 390` at a 390 px viewport.

## Demo sandbox

One click opened `/?demo=1`. The page immediately showed the `northstar-api` sample and the persistent **“Demo — sample data, nothing is saved”** label with **Reset demo** and **Start for real**.

Running the sample displayed TypeScript and Rust language servers, Prettier and Rustfmt, 42 passing tests, and an Ed25519 tamper check. Reload retained the demo label. A fresh context was seeded with `real:lsp-readiness-check=sentinel`; run, reload, and reset preserved that value. Start for real removed only `demo:lsp-readiness-check`. No cross-origin request occurred.

The CLI sample also produced a ready JSON report and `verify --json` returned `{"valid":true,"algorithm":"Ed25519"}`.

## Declared claims

A fresh clone at documentation SHA `2006c75` received `npm ci`. Every exact command in `.factory/claims.json` then exited 0. Each id has exactly one `@claim:<id>` test.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `sample-probe` | Pass | The fixture ran 42 tests; generated and published digests matched; the browser showed both LSPs, both formatters, tests, and signature evidence. |
| `local-operation` | Pass | Demo traffic stayed same-origin; the executable fake runtime captured the digest-pinned, network-disabled, read-only, capability-dropped invocation; source stayed unchanged. |
| `signed-packet` | Pass | A fresh Ed25519 readiness report verified. Tamper rejection also passed in the Rust suite. |
| `offline-demo` | Pass | A separate service-worker context reloaded `/demo` offline. |
| `no-account` | Pass | Browser and CLI demos completed without credentials or authentication requests. |
| `no-tool-install` | Pass | Installer/tool traps were not called; source stayed unchanged. |
| `no-dependency-install` | Pass | Dependency-manager traps were not called; source stayed unchanged. |
| `noninteractive-ci` | Pass | `check`, `container`, `demo`, and `verify` completed with stdin closed. |
| `signing-key-permissions` | Pass | A first Linux check created its key with mode `0600`. |

The landing page, legal pages, demo, README, and CLI help were cross-checked against the manifest. Every public reliance claim maps to a passing claim test. No unlisted, false, incomplete, or untested claim was found.

## Clean checkout and consumer package

| Check | Result |
| --- | --- |
| `npm ci` | Pass; 23 packages installed, 0 vulnerabilities. |
| All nine declared claim commands | Pass. |
| `npm test` | Pass; 11 Rust tests and 27 Playwright tests. |
| `npm run build` | Pass; `dist/site/` produced. |
| `cargo fmt --check` | Pass. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Pass. |
| `cargo package --allow-dirty` | Pass; 58 files, 80.0 KiB compressed; package verification compiled. |
| Fresh consumer install | Pass from `target/package/lsp-readiness-check-0.1.1` with `--locked`. |

The installed CLI reported version 0.1.1 and useful help for `check`, `container`, `demo`, and `verify`. Its demo report verified. Missing `--image` and mutable `ubuntu:latest` inputs both failed closed with exit 2 and actionable messages. Unit and integration coverage also passed for an empty repository, signature tampering, non-ready results, external relative and absolute symlinks, host-side signing, and container-argument isolation.

## Live browser, accessibility, privacy, and recovery

- Live Playwright: **27/27 passed** after the documented build prerequisite. It includes Axe WCAG A/AA checks on `/`, `/demo`, `/privacy`, `/terms`, and the 404; no serious or critical issue was found.
- `verify-url.sh`: pass; HTTP 200, 561 ms load, no console errors, `lang=en`, one H1, main landmark, complete image alt coverage, and named buttons.
- Keyboard: Tab reached the skip link; Enter focused main; the next Tab focused the sample action. The focus outline is 3 px and has at least 3:1 contrast against every surface where it appears.
- Navigation: click, Back, and Forward changed the route title and moved focus to the new H1. All public routes have one H1 and their own title, description, canonical, Open Graph, and Twitter metadata.
- Reduced motion: the media query matched, scroll behavior became `auto`, and animation/transition durations became `0.001ms`.
- Mobile and zoom: every visible link/button met 44 px in the suite. At a 390 px viewport with 200% root text size, the page retained its H1 and primary action without horizontal overflow.
- Privacy: demo traffic was same-origin, the CSP limits `connect-src` to self, and no analytics, third-party script, or third-party font request appeared.
- Offline/update: the live service worker reloaded the demo offline and removed the prior release cache.
- Links: all discovered HTTP links on home, demo, legal, and 404 pages resolved. The privacy and support contacts are explicit `mailto:` links.
- Recovery: an unknown URL deliberately returned HTTP 404 and rendered the shared skip link, header, named navigation, one main/H1, footer, plain “Page not found” text, and a working return-home action.

Fresh Lighthouse 12.8.2 mobile results were **98 performance, 100 accessibility, 100 best practices, and 100 SEO**. FCP was 0.97 s, LCP 1.81 s, TBT 136 ms, and CLS 0. Initial gzip JavaScript is 5.12 KiB and CSS is 3.68 KiB.

## Deployment identity

The canonical `/work/repo` build and live deployment have identical SHA-256 values:

| Artifact | SHA-256 |
| --- | --- |
| Root HTML | `b696919082e4626639113ce651b5d9719252c243baa2a4860b01dd87b276014d` |
| Static 404 HTML | `f87d4e8ad1057d5b63d61b935ca3405755a4b2fae2dd3dfb58331ec122c559cb` |
| JavaScript | `f2bed6f847af466fb0093baee56fa760f99544cd442ae595ed70fc678d7be44f` |
| CSS | `d9ab7665da6abd91151642632867314f15636b53805bc7b2f738fdeba2ff5639` |
| Linux CLI | `4e7cc788275bd352a5c91b60ca7606d9b9863a0f33d3205c5cc15b4b03dd9fbd` |

The live binary reports version 0.1.1 and completed demo/verify. Commits after implementation `7481781` change only documentation and factory records; the reviewed documentation baseline is `2006c75`. Static Web App deployment `fb33c0f7-9af2-428f-969a-8a41f8f7373e` remains the recorded deployment.

## Earlier findings — current disposition

| Earlier finding | Fresh disposition |
| --- | --- |
| Verification 1: false 42-test demo and stale digest | Fixed. The executable fixture reports 42 tests; generated and published digests match; the report verifies. |
| Verification 1: unavailable paid checkout | Fixed by removal. M1 presents no paid offer, auth, checkout, or billing endpoint. |
| Verification 1: mobile targets below 44 px | Fixed. Live 390 px landing and demo target checks pass. |
| Verification 1: mutable image accepted | Fixed. Clean consumer and live binaries reject a mutable tag before runtime startup. |
| Verification 1: unknown route returned 200 | Fixed. The unknown route returns an intentional HTTP 404. |
| Verification 1: strict Clippy failure | Fixed. Strict Clippy exits 0. |
| Verification 2: default probe ran repository tools on the host | Fixed. The normal command’s executable isolation test captures the locked-down container invocation. Existing real-Docker evidence covers ready, non-ready, timeout, and runtime-error outcomes. |
| Verification 2: directory symlink escape | Fixed. Relative and absolute external-directory symlink regressions pass. |
| Review 1 F-1-1: no-account claim absent | Fixed. `no-account` is declared once and passes. |
| Review 1 F-1-2: no-tool-install claim absent | Fixed. `no-tool-install` is declared once and passes. |
| Review 1 F-1-3: no-dependency-install claim absent | Fixed. `no-dependency-install` is declared once and passes. |
| Review 1 F-1-4: noninteractive-CI claim absent | Fixed. `noninteractive-ci` is declared once and passes. |
| Review 1 F-1-5: key-permission claim absent | Fixed. `signing-key-permissions` is declared once and passes. |
| Review 1 F-1-6: “preflight” jargon headings | Fixed. The live copy uses “repository check.” |
| Review 1 F-1-7: slogan heading | Fixed. The section is named “Signed JSON readiness report.” |
| Review 2 F-2-1: route social metadata stayed on home values | Fixed. Raw responses and hydrated routes carry their own metadata. |
| Review 2 F-2-2: unexplained cryptographic output wording | Fixed. The user outcome is “readiness report”; Ed25519 is a parenthetical implementation detail. |
| Review 2 F-2-3: undefined image terminology | Fixed. The live copy explains that the user chooses an exact SHA-256-addressed development image. |
| Verification 5 F-5-1: incomplete and metaphorical 404 | Fixed. The live HTTP 404 has the shared structure, plain heading, and recovery action. |
| Verification 5 F-5-2: inconsistent “packet” terminology | Fixed. Visitor-facing demo, privacy, README, help, and completion output use “readiness report.” |

No previous finding recurred.

## Current milestone and external dependencies

M1 is the only evaluated milestone. It is a Static Web App plus a local CLI, with no Container App, backend, account, tenant data, product database, paid tier, or product API. Therefore tenant isolation, restart persistence, `/data`, health, 429/`Retry-After`, and billing checks are not applicable to the current public product.

The unchanged real-Docker matrix remains prior accepted evidence; this verifier has neither Docker nor Podman installed and did not invent a fresh engine result. Podman and arbitrary customer-image compatibility remain external environment work. M2 separately depends on Entra CIAM, GitHub App registration, Sociobot subscription registration, a product-scoped SQLite `/data` service, tenant isolation, export/delete, health, and rate limiting.

No AI feature is warranted for M1: the job is deterministic local verification, and a model call would weaken its source-privacy boundary. The signed JSON download already supplies the useful export implied by the brief.

## Evidence

Primary logs and browser captures are under `/work/.evidence/verification-6/`: `clean-claims.log`, `clean-quality.log`, `consumer-install.log`, `live-playwright.log`, `fresh-browser.json`, `accessibility-recovery.json`, `history-focus.json`, `link-crawl.json`, `live-runtime.log`, `lighthouse.json`, and `verify-url/verify.json`.

## Final result

**PASS — 0 findings, 0 untested claims.**
