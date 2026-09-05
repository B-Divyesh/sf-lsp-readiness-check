# Review 3 — verify tooling before an agent edits

- Work order: `lsp-readiness-check-review-3`
- Current milestone: M1 — free local CLI and static demo
- Implementation reviewed: `748178140e4f46e75bc596086f09da9bfd3605ba`
- Documentation baseline reviewed: `ee34274fb661cd0ad2587ee298d6f1f8c08a2a4c`
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Reviewed: 2026-09-05 UTC
- Verdict: **PASS**
- Findings: **0**
- Untested claims: **0**

## Decision

**PASS — zero findings of every severity and zero untested claims.** M1 works end to end as a free local readiness CLI and one-click static demo. The live site and downloadable binary match the reviewed implementation. Every declared claim command passed from a fresh GitHub checkout, and the packaged CLI worked from a separate consumer prefix.

M2 is not part of this verdict. Accounts, private CI, billing, tenant data, SQLite persistence, health, and rate limiting are planned capabilities. The current site does not present them as available.

## First screen before scrolling

Fresh Chromium contexts opened the live page at 1366×900 and 390×844. Both started at `scrollY = 0`, showed the full primary action, fit their viewport, and logged no console or page error.

| Question | Live answer |
| --- | --- |
| Job | “Verify tooling before an agent edits” |
| Audience | “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.” |
| First action | “Try it with sample data” with “See a finished probe in one click.” |

The title, **LSP Readiness Check — verify repository tooling**, names the job in plain words. The page uses the documented survey-sheet visual system without metaphor headings or generic template copy.

## Demo and real-data boundary

The first action entered the sample in one click on desktop and phone. The resulting screen already showed a completed `northstar-api` readiness result with TypeScript and Rust language servers, Prettier and Rustfmt, 42 passing tests, and an Ed25519 signature.

The persistent label read **Demo — sample data, nothing is saved** and kept **Reset demo** and **Start for real** available. A seeded `real:lsp-readiness-check=sentinel` value survived the sample run, reset, and demo exit. Reset restored the separate `demo:lsp-readiness-check` sample state. Start for real removed only the demo key. All observed browser requests stayed on the product origin.

The CLI demo also used a new operating-system temporary directory, printed the readiness-report path, and produced a report accepted by `verify --json`.

## Declared claims

The clean checkout was a fresh GitHub clone at documentation baseline `ee34274`. `npm ci` installed the documented prerequisites first. Each manifest test string was then run exactly as declared; each exited 0. Every claim id occurs exactly once in the test suite.

| Claim | Exact command | Result and observed evidence |
| --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Pass. The fixture ran 42 tests; generated and published digests matched; both language servers, formatters, signed output, and browser result passed. |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Pass. Browser traffic stayed same-origin; the normal command supplied the pinned image, no-network, read-only, dropped-capability, no-new-privileges, and source-mount contract; symlink tests passed. |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Pass. A fresh Ed25519 readiness report verified; the Rust suite also rejected changed signed content. |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | Pass. A separate browser context reloaded `/demo` offline after its first visit. |
| `no-account` | `npm test -- --grep @claim:no-account` | Pass. Browser and CLI demos completed without credentials, cookies, or an authentication request. |
| `no-tool-install` | `npm test -- --grep @claim:no-tool-install` | Pass. Tool and installer traps did not run; source stayed unchanged. |
| `no-dependency-install` | `npm test -- --grep @claim:no-dependency-install` | Pass. Dependency-installer traps did not run; source stayed unchanged. |
| `noninteractive-ci` | `npm test -- --grep @claim:noninteractive-ci` | Pass. `check`, `container`, `demo`, and `verify` completed with stdin closed. |
| `signing-key-permissions` | `npm test -- --grep @claim:signing-key-permissions` | Pass. A first Linux check created a mode-0600 signing key. |

Landing, demo, privacy, terms, README, help, and normal completion copy were cross-checked against the manifest. The public promises map to the sample, local-operation/privacy, signature, offline, no-account, no-install, noninteractive, and key-permission claims above. No false, incomplete, missing, duplicate, or untested claim was found.

## Clean checkout and installed CLI

| Check | Result |
| --- | --- |
| `npm ci` | Pass; 23 packages installed and 0 vulnerabilities. |
| `npm test` | Pass; 11 Rust tests and 27 Playwright tests. |
| `npm run build` | Pass; `dist/site/` produced. |
| TypeScript checks | Pass as part of `npm test`. |
| `cargo fmt --check` | Pass. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Pass. |
| `cargo package --allow-dirty` | Pass; 58 files, 80.0 KiB compressed, and package verification compiled. |
| `npm audit --audit-level=high` | Pass; 0 vulnerabilities. |

The verified package was installed into a fresh consumer prefix with `cargo install --path target/package/lsp-readiness-check-0.1.1 --root <temporary-prefix> --locked`. The installed artifact provided useful help, reported version 0.1.1, ran the bundled demo, and verified its generated report.

Recovery paths failed closed with actionable output: a missing image, mutable `ubuntu:latest`, and a missing report each exited 2. Unit and integration tests also cover an empty repository, changed signed content, a non-ready result, runtime failure, and relative and absolute directory-symlink boundaries. The live downloadable binary independently completed demo and verify.

## Live browser, accessibility, privacy, offline, and links

- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test`: **27/27 passed**.
- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, 566 ms load, no console errors, `lang=en`, one H1, one main landmark, complete image alt text, and named buttons.
- Axe WCAG A/AA checks found no serious or critical issue on `/`, `/demo`, `/privacy`, `/terms`, or the 404.
- Keyboard use reached the skip link, main content, and primary sample action. The action had a visible 3 px solid focus outline. Direct SPA navigation, Back, and Forward moved focus to the new H1 and restored the top position.
- At 390 px, the page had no horizontal overflow and all checked links and buttons met 44 px targets. At 200% text size, the H1 and primary action remained available without horizontal overflow.
- Reduced-motion matching was active in the dedicated context; animation and transition durations reduced to effectively instant and scroll behavior was `auto`.
- Offline demo reload and service-worker cache replacement passed in fresh contexts.
- `/`, `/demo`, `/privacy`, and `/terms` returned 200 with route-specific titles and social metadata before and after hydration. Public assets, sample JSON, download, and the external Param Factory link resolved.
- The unknown address deliberately returned HTTP 404. Its designed page retained the skip link, header, named navigation, one main/H1, footer, plain **Page not found** heading, and return-home action. The 404 status is expected and is not a defect.
- Response headers include HSTS, `nosniff`, a strict referrer policy, restrictive permissions policy, and a response-header CSP with same-origin connections and `frame-ancestors 'none'`.

Privacy behavior matches the current no-server product: there is no analytics, authentication, billing, or third-party runtime script; browser sample traffic was same-origin; the CLI has no telemetry or network client; and active repository commands receive the network-disabled container boundary.

## Performance and deployment identity

Fresh Lighthouse 12.8.2 mobile scores were **100 performance, 100 accessibility, 100 best practices, and 100 SEO**. FCP was 0.76 s, LCP 1.81 s, total blocking time 16 ms, and CLS 0. Initial gzip JavaScript is 5.12 KiB and CSS is 3.68 KiB; the self-hosted font is 67,304 bytes and hero WebP is 120,118 bytes.

The current clean build and live deployment have identical SHA-256 values:

| Artifact | SHA-256 |
| --- | --- |
| Root HTML | `b696919082e4626639113ce651b5d9719252c243baa2a4860b01dd87b276014d` |
| Static 404 HTML | `f87d4e8ad1057d5b63d61b935ca3405755a4b2fae2dd3dfb58331ec122c559cb` |
| JavaScript | `f2bed6f847af466fb0093baee56fa760f99544cd442ae595ed70fc678d7be44f` |
| CSS | `d9ab7665da6abd91151642632867314f15636b53805bc7b2f738fdeba2ff5639` |
| Linux CLI | `4e7cc788275bd352a5c91b60ca7606d9b9863a0f33d3205c5cc15b4b03dd9fbd` |

No product-code file differs between implementation `7481781` and the reviewed documentation baseline. Later commits change README/factory records only, so no new product image is required.

## Earlier findings — fresh disposition

| Earlier finding | Current proof | Result |
| --- | --- | --- |
| Verification 1 high: sample claimed 42 tests without producing them | Fresh fixture execution reports 42; generated and published digests match; signed result verifies. | Fixed |
| Verification 1 high: advertised paid checkout returned 404 | M1 presents no paid offer, price, checkout, billing call, or paid entitlement. | Fixed by removal |
| Verification 1 medium: mobile targets below 44 px | Live 390 px landing and demo measurements pass. | Fixed |
| Verification 1 medium: mutable image accepted | Installed and live binaries reject `ubuntu:latest` before runtime startup. | Fixed |
| Verification 1 low: unknown routes returned 200 | Fresh unknown address returns the intentional designed HTTP 404. | Fixed |
| Verification 1 low: strict Clippy failed | Fresh strict Clippy exits 0. | Fixed |
| Verification 2 high: default check ran repository tools on the host | Normal-command tests capture the locked-down runtime invocation; accepted real-Docker evidence covers ready, non-ready, timeout, and runtime-error outcomes. | Fixed |
| Verification 2 medium: repository walk followed directory symlinks | Relative and absolute external-directory symlink tests pass. | Fixed |
| Review 1 F-1-1: no-account claim was absent | Exactly one declared `no-account` test passed. | Fixed |
| Review 1 F-1-2: no-tool-install claim was absent | Exactly one declared `no-tool-install` test passed. | Fixed |
| Review 1 F-1-3: no-dependency-install claim was absent | Exactly one declared `no-dependency-install` test passed. | Fixed |
| Review 1 F-1-4: noninteractive-CI claim was absent | Exactly one declared `noninteractive-ci` test passed. | Fixed |
| Review 1 F-1-5: key-permission claim was absent | Exactly one declared `signing-key-permissions` test passed. | Fixed |
| Review 1 F-1-6: “preflight” jargon headings | Live copy uses “repository check.” | Fixed |
| Review 1 F-1-7: slogan heading | Live section is named “Signed JSON readiness report.” | Fixed |
| Review 2 F-2-1: routed social metadata stayed on home values | Raw live responses and hydrated routes carry their own title, description, canonical, Open Graph, and Twitter values. | Fixed |
| Review 2 F-2-2: unexplained cryptographic output wording | Live copy leads with “readiness report” and explains tamper detection; Ed25519 is secondary detail. | Fixed |
| Review 2 F-2-3: undefined image terminology | Live and README copy say the user chooses an exact image and explain its SHA-256 address. | Fixed |
| Verification 5 F-5-1: incomplete, metaphorical 404 | The live HTTP 404 uses the shared structure, plain heading, and recovery action. | Fixed |
| Verification 5 F-5-2: inconsistent “packet” terminology | Visitor-facing demo, privacy, README, help, and completion output use “readiness report.” | Fixed |

Verification 3, verification 4, and verification 6 recorded no additional findings. No earlier finding recurred.

## Current milestone and external dependencies

M1 is the only reviewed milestone. It is a static site plus a local CLI. It has no backend, account, tenant, product database, paid tier, or product API. Tenant isolation, restart persistence, `/data` SQLite, health, 429/`Retry-After`, billing, and subscription entitlement therefore do not apply to the current public product.

The unchanged product-scoped real-Docker matrix remains accepted evidence for the current container path. Docker and Podman are absent from this reviewer container, so no new real-engine result is claimed. Customer Podman compatibility and arbitrary customer development images remain external environment work.

M2 separately depends on factory-provisioned Entra CIAM, GitHub App registration, Sociobot subscription registration, a product-scoped `/data` SQLite service, tenant isolation, export/delete, health, and rate limits. None is presented as shipped.

No AI feature is missing from M1. The job is deterministic local verification, and a model call would weaken the source-privacy boundary. The signed JSON readiness report already provides the useful export.

## Evidence

Fresh browser screenshots and machine-readable checks are under `/work/.evidence/review-3/`, including `desktop-first-screen.png`, `phone-first-screen.png`, `fresh-browser.json`, `accessibility-recovery.json`, `history-focus.json`, `lighthouse.json`, and `verify-url-4/verify.json`.

## Final result

**PASS — 0 findings, 0 untested claims.**
