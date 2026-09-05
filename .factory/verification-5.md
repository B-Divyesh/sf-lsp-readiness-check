# Verification 5 — verify tooling before agent edits

- Work order: `lsp-readiness-check-verify-5`
- Current milestone: M1 — free local CLI and static demo
- Implementation reviewed: `b3714c16ec78b14d5d403d7eaa98e5ac0b27ee02`
- Documentation reviewed: `35b58749f91feac2bc155534be303167e6ad8fd5`
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Verified: 2026-09-05 UTC
- Verdict: **FAIL**
- Findings: **2 minor**
- Untested claims: **0**

## Decision

**FAIL.** The M1 CLI, one-click demo, all nine declared claims, installed package, live browser suite, accessibility checks, and performance checks pass. Two public-page defects remain. The live 404 does not use the required site structure and uses metaphor copy. Privacy/demo copy also uses the old term “packet” instead of the documented plain term “readiness report.” This verification cannot return PASS until both findings are fixed and rechecked.

## First screen before scrolling

Fresh Chromium contexts opened the live root at 1366×900 and 390×844. Both started at `scrollY = 0`; the primary action’s full bounding box was inside each viewport.

| Question | Live answer | Result |
| --- | --- | --- |
| What is the job? | “Verify tooling before an agent edits” | Pass |
| Who is it for? | “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.” | Pass |
| What should happen first? | “Try it with sample data” and “See a finished probe in one click.” | Pass |

The phone page had `scrollWidth = 390` at a 390 px viewport. Neither context logged a console or page error. Screenshots are `/work/.evidence/lsp-readiness-check-verification-5-desktop.png` and `/work/.evidence/lsp-readiness-check-verification-5-phone.png`.

## Findings

### F-5-1 — Minor: the live 404 omits the standard page structure and uses metaphor copy

**Evidence:** A direct request to `/does-not-exist` correctly returns HTTP 404. The returned HTML has one `<main>`, a useful “Return home” link, and the correct title, “Page not found — LSP Readiness Check.” It has no `header`, `nav`, or `footer`. Its visible label is “Map edge · 404” and its H1 is “This route is not on the map.”

**Why this is a finding:** The deliberate 404 status is correct and is not the defect. The site-structure contract requires the consistent header and footer on every route. The plain-words contract forbids metaphor headings. The static-host 404 response is the page visitors actually receive for an unknown address, so the separate client-rendered “Page not found” route does not repair this response.

**Required repair:** Make the static 404 use the standard header, navigation, skip link, footer, and plain “Page not found” heading while retaining HTTP 404 and the return-home action.

### F-5-2 — Minor: public pages use two names for the signed output

**Evidence:** `.factory/copy-audit.md` sets “readiness report” as the required term for the signed JSON output. The live privacy page instead says it builds a “capability packet,” and the demo terminal says “Sample packet is stored only in this demo.” The README also calls it a “temporary packet” and later “packets,” after introducing “signed JSON readiness report.”

**Why this is a finding:** The plain-words contract requires one word for one concept everywhere. “Capability packet” was the unexplained term in earlier finding F-2-2. The main landing repair is good, but the inconsistent term remains on public supporting surfaces.

**Required repair:** Use “readiness report” in visitor-facing privacy, demo, and README copy. Keep `packet` only where it is an unavoidable code/schema identifier, with a plain definition nearby.

## Declared claims

After `npm ci` in a fresh GitHub checkout at documentation SHA `35b5874`, every command from `.factory/claims.json` was run exactly as declared. Each exited 0. Each claim id occurs once in the test suite.

| Claim | Result | Observable evidence |
| --- | --- | --- |
| `sample-probe` | Pass | The shipped fixture ran 42 tests; TypeScript/Rust servers, formatters, digest, and signed sample result matched. |
| `local-operation` | Pass | Demo traffic stayed same-origin; the executable runtime captured the pinned image and locked-down container arguments; source stayed unchanged. |
| `signed-packet` | Pass | A new Ed25519 result verified; an independently tampered result exited 2. |
| `offline-demo` | Pass | A fresh service-worker context reloaded `/demo` offline. |
| `no-account` | Pass | Browser and CLI demos completed without credentials or authentication requests. |
| `no-tool-install` | Pass | Tool/installer traps did not run; source stayed unchanged. |
| `no-dependency-install` | Pass | Dependency-installer traps did not run; source stayed unchanged. |
| `noninteractive-ci` | Pass | `check`, `container`, `demo`, and `verify` completed with stdin closed. |
| `signing-key-permissions` | Pass | A first Linux check created the key with mode 0600. |

Full output: `/work/.evidence/lsp-readiness-check-verification-5-claims.log`.

## Clean checkout and installed CLI

| Check | Result |
| --- | --- |
| `npm ci` | Pass; 23 packages installed and 0 vulnerabilities reported. |
| `npm test` | Pass; 11 Rust tests and 25 Playwright tests. |
| `npm run build` | Pass as part of `npm test`; `dist/site/` produced. |
| `cargo fmt --check` | Pass. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Pass. |
| `cargo package --allow-dirty` | Pass; 58 files, 79.7 KiB compressed, verification build passed. |
| Clean consumer install | Pass with `cargo install --path target/package/lsp-readiness-check-0.1.1 --root <temporary-prefix> --locked`. |

The installed CLI exposed useful `--help`, reported version 0.1.1, ran the bundled demo, and verified its result. Tampering, a missing result file, a missing image, and mutable `ubuntu:latest` all failed closed with exit 2 and actionable text. The live downloadable binary also ran the demo and verified its generated result.

Quality log: `/work/.evidence/lsp-readiness-check-verification-5-quality.log`.

## Live browser, accessibility, privacy, and performance

- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test`: **25/25 passed**. This covers one-click sample entry, realistic populated output, persistent demo label, reset/exit isolation, same-origin requests, offline reload, service-worker update, route metadata, internal links, mobile targets, keyboard operation, and Axe WCAG A/AA serious/critical checks.
- An independent demo check seeded `real:lsp-readiness-check=sentinel`. Run and reset preserved it. “Start for real” removed only `demo:lsp-readiness-check`. All recorded requests stayed on the product origin.
- Keyboard order reached the skip link, main, then the primary action. The focused action had a `3px` solid visible outline. Back navigation restored the home route and main focus.
- At 200% root text size, the 390 px page kept its H1 and action visible without horizontal overflow.
- With reduced motion enabled, media-query matching was true, motion durations were `0.000001s`, and scroll behavior was `auto`.
- `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, 569 ms load, no console errors, `lang=en`, one H1, main landmark, alt coverage, and named buttons.
- Lighthouse 12.8.2 mobile: performance 99, accessibility 100, best practices 100, SEO 100; FCP 0.9 s, LCP 1.8 s, total blocking time 90 ms, CLS 0.
- Initial gzip JavaScript is 5.13 KiB and CSS is 3.68 KiB. The self-hosted font is 67,304 bytes and hero WebP is 120,118 bytes.
- `/`, `/demo`, `/privacy`, and `/terms` returned 200. The intentional unknown route returned 404. Public assets, sample JSON, and Linux download returned 200. Legal pages had their own titles, one H1, main landmark, and no serious/critical Axe issue.
- Response headers include HSTS, `nosniff`, strict referrer policy, restrictive permissions policy, and a response-header CSP with same-origin connections and `frame-ancestors 'none'`. Hashed assets use one-year immutable caching.

Evidence: `/work/.evidence/lsp-readiness-check-verification-5-live-playwright.log`, `/work/.evidence/lsp-readiness-check-verification-5-cold-browser.json`, `/work/.evidence/lsp-readiness-check-verification-5-url/verify.json`, and `/work/.evidence/lsp-readiness-check-verification-5-lighthouse.json`.

## Earlier findings

| Earlier issue | Fresh disposition |
| --- | --- |
| False 42-test demo and stale digest | Fixed; fixture execution, generated/published digest comparison, signature verification, browser output, and live binary demo passed. |
| Undeployed private-CI checkout | Fixed by removal; no paid offer, checkout, auth, or billing request is present in M1. |
| Mobile targets below 44 px | Fixed; live phone tests passed for landing and demo. |
| Mutable image accepted | Fixed; installed CLI rejected `ubuntu:latest` before runtime startup. |
| Unknown route returned 200 | Fixed; live unknown route returns 404. F-5-1 concerns its copy and required structure, not its status. |
| Strict Clippy failure | Fixed; strict Clippy passed. |
| Default probe ran repository tools on the host | Fixed; executable isolation test captured the normal command’s container invocation. The accepted Docker matrix remains the real-engine evidence. |
| Directory symlink escape | Fixed; relative and absolute external-directory symlink regressions passed. |
| Five unlisted claims from review 1 | Fixed; each now has one manifest entry and one passing tagged test. |
| “Preflight” and slogan headings | Fixed; current headings name the repository check and readiness report. |
| Route social metadata | Fixed; direct raw responses and hydrated pages passed for home, demo, privacy, and terms. |
| Unexplained signed-output and image wording | Landing and install explanation are fixed. F-5-2 records the remaining terminology inconsistency on supporting surfaces. |

The assignment records that Docker passed ready, non-ready, LSP-timeout, and runtime-error cases at the implementation SHA. The referenced prior `/work/.evidence` matrix files were not mounted in this fresh worker, and neither Docker nor Podman is installed here. The current fake-runtime isolation claim was rerun. The implementation has not changed since that accepted Docker run.

## Deployment identity

The live root HTML, JavaScript, and CSS SHA-256 values exactly match the clean build. The live downloadable binary and a build made from the same `/work/repo` path both have SHA-256 `ce2470c3c4f1d51c08031f8c228736b08a491aa840cf18615dd786cded503c9a`. The binary reports version 0.1.1.

Only `.factory/handoff.md`, `.factory/plan.md`, and `README.md` changed between implementation `b3714c1` and documentation `35b5874`; no later product runtime change requires another image.

## Current milestone and external dependencies

M1 is the only evaluated milestone. It ships a free local CLI and static demo. There is no backend, tenant state, account, paid plan, or product API, so tenant isolation, restart persistence, health, and 429/`Retry-After` checks are not applicable to M1.

Customer Podman validation remains external. Customer CI also supplies a digest-pinned Linux x86-64 development image with compatible glibc and the repository’s tools. M2 identity, GitHub App, subscription, product API, and SQLite `/data` foundation are planned and not presented as shipped capabilities.

## Final result

**FAIL — 2 findings, 0 untested claims.**
