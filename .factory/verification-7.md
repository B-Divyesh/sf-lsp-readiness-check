# Verify the M2 private CI foundation — verification 7

- **Date:** 2026-09-06
- **Live URL:** <https://lsp-readiness-check.sociobot.in>
- **Milestone:** M2 product-owned foundation
- **Implementation reviewed:** `2428fcb82bd9af430b8bc98bb1d01421c5660eff`
- **Verification-only commit:** `a011cc3e31863c8d56b01ce1b24701919bb3af7e`
- **Documentation reviewed:** `8da11e5a107bdfffa18489164266931dc17605bc`
- **Verdict:** **PASS**
- **Findings:** **0**
- **Untested claims:** **0**

## Decision

**PASS — zero findings of every severity and zero untested claims.** The deployed M2 product-owned API foundation, accepted CLI, static site, and isolated demo work within the current milestone's stated boundary. All 14 declared claim commands passed from a fresh checkout. The complete local suite passed, the packaged CLI worked in a separate consumer prefix, and the live suite passed with only the six intentionally local authenticated-API tests skipped.

Hosted CIAM sign-in, a real GitHub App installation, and the recurring subscription lifecycle are not configured and are not presented as working. They remain three named operator dependencies, recorded separately below.

## First screen before scrolling

Fresh Chromium contexts were used at 1440 × 900 and 390 × 844. Both began at scroll position zero.

| Question | Live answer | Result |
| --- | --- | --- |
| What is the job? | “Verify tooling before an agent edits” | Pass |
| Who is it for? | “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.” | Pass |
| What should happen first? | “Try it with sample data” and “See a finished probe in one click.” | Pass |

The action was inside the first viewport on both sizes. The page title is “LSP Readiness Check — verify repository tooling.” The wording names the job, audience, and next action without metaphor.

## Demo sandbox

- The first action entered `/?demo=1` in one click.
- The populated result immediately showed `5/5`, TypeScript and Rust language servers, Prettier and Rustfmt, 42 passing tests, and an Ed25519 tamper check.
- “Demo — sample data, nothing is saved” remained visible after reload.
- A separate real-data sentinel was preserved through **Reset demo** and **Start for real**. Reset recreated only `demo:lsp-readiness-check`; exit removed only that key.
- Desktop and phone runs made no cross-origin request and logged no console or page error.
- The offline claim passed in its own browser context after service-worker readiness. The update regression removed the old cache and retained only `lsp-readiness-v4`.

## Declared claims

Each manifest command was run exactly as written from a fresh GitHub checkout of documentation head `8da11e5`. Every claim tag occurs exactly once in the test sources.

| Claim | Exact command | Result and observed evidence |
| --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Pass — the fixture executed 42 tests, both language servers and formatters were ready, and the displayed signed report matched the generated sample. |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Pass — the demo stayed same-origin; the normal command used the pinned, network-disabled, read-only, capability-dropped container contract and skipped source symlinks. |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Pass — the generated Ed25519 report verified. |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | Pass — a separate browser context reloaded the demo offline. |
| `no-account` | `npm test -- --grep @claim:no-account` | Pass — browser and CLI demos ran without credentials or an authentication request. |
| `no-tool-install` | `npm test -- --grep @claim:no-tool-install` | Pass — command traps did not run and source stayed unchanged. |
| `no-dependency-install` | `npm test -- --grep @claim:no-dependency-install` | Pass — dependency installer traps did not run and source stayed unchanged. |
| `noninteractive-ci` | `npm test -- --grep @claim:noninteractive-ci` | Pass — `check`, `container`, `demo`, and `verify` completed with stdin closed. |
| `signing-key-permissions` | `npm test -- --grep @claim:signing-key-permissions` | Pass — the first normal check created a Linux mode-0600 key. |
| `tenant-isolation` | `npm test -- --grep @claim:tenant-isolation` | Pass — two organizations could not list, export, read, or update one another's repository, policy, or run data. |
| `packet-upload-no-source` | `npm test -- --grep @claim:packet-upload-no-source` | Pass — a valid signed report was accepted; wrong token, source-shaped fields, secret-like evidence, and a request above 64 KB were rejected. Rejected source markers were absent from export. |
| `export-delete` | `npm test -- --grep @claim:export-delete` | Pass — an owner exported its repository and policy, deleted the organization, and received an empty repository list afterward. |
| `rate-limit` | `npm test -- --grep @claim:rate-limit` | Pass — repeated authenticated requests returned 429 and a positive `Retry-After`. A fresh live burst also returned 429 with `Retry-After: 39`. |
| `subscription-registration-pending` | `npm test -- --grep @claim:subscription-registration-pending` | Pass — the exact $49 per repository per month offer is labeled unavailable, and no checkout or Sociobot billing link is exposed. |

Landing, demo, legal, account-status, README, CLI help, and completion copy were cross-checked against the manifest. Current customer-facing promises map to these 14 tests. Developer and operator descriptions were also exercised by the full suite, package check, direct API checks, and restart check below. No false, incomplete, missing, duplicate, or untested public claim was found.

## Clean checkout and installed CLI

| Check | Result |
| --- | --- |
| `npm ci` | Pass — 23 packages installed; zero vulnerabilities. |
| All 14 exact claim commands | Pass — 14/14. |
| `npm test` | Pass — 4 API unit tests, 11 CLI/library tests, and 38 Playwright tests. |
| `npm run build` | Pass — wrote `dist/site/`; JS 30.45 KB (9.54 KB gzip), CSS 14.39 KB (4.11 KB gzip). |
| `cargo fmt --all --check` | Pass. |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | Pass. |
| `npm audit --audit-level=high` | Pass — zero vulnerabilities. |
| `cargo package -p lsp-readiness-check --locked --allow-dirty` | Pass — packaged and verified version 0.1.2. |
| Separate consumer prefix | Pass — package install, `--help`, `--version`, normal demo, generated-report verification, missing-report handling, and unavailable-runtime handling behaved as documented. |
| Live downloadable CLI | Pass — version 0.1.2 ran the 42-test demo, verified its generated report, and rejected `ubuntu:latest` with exit code 2 before runtime startup. |

The accepted real-Docker M1 matrix was not repeated or downgraded. It already covers ready, non-ready, language-server timeout, and runtime-error outcomes with unchanged source checksums. Podman and arbitrary customer images remain compatibility limits, not current claims.

## API and durable state

- Live `GET /healthz` returned 200 with `database: ok` and schema version 1.
- Live `/api/v1/config` returned `identity_configured: false`, `github_app_configured: false`, and `subscription_configured: false`.
- Live `/api/v1/session` without a token returned 401 with a plain recovery message.
- A live burst reached 429 and included a positive `Retry-After` header.
- The API root returned an empty HTTP 404 with security headers. This is the deliberate, documented result; `/healthz` is the probe.
- CORS responses name only the product origin. Requests from another origin cannot read the response.
- A fresh local API created a tenant repository in a new SQLite file, stopped, restarted on the same file, returned healthy schema version 1, and returned the same tenant row. No credential is recorded in this report.
- The full suite separately passed two-organization isolation, report-token rejection, strict upload/schema limits, export/delete, reversible migration, SQLite reopen, backup, and restore paths.

The live tenant row and hosted identity paths cannot be exercised until CIAM registration. This is an explicit unavailable dependency, not an untested shipped claim.

## Live browser, accessibility, privacy, and recovery

- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test`: **32 passed, 6 skipped**. The six skips are the local test-identity API tests; release intentionally disables that identity mode.
- The worker URL verifier returned 200 in 589 ms, with `lang=en`, one H1, a main landmark, no missing image alt text, no unlabeled button, and no console error.
- Playwright Axe found zero serious or critical issues across `/`, `/demo`, `/privacy`, `/terms`, `/sign-in`, `/app`, `/app/repositories`, `/app/billing`, and the 404.
- Keyboard navigation reached the skip link, main content, and primary demo action. Native buttons and links retained visible focus and standard keyboard behavior.
- At 390 px, all tested public, legal, account, billing, and 404 routes had no horizontal overflow. Every visible link and button was at least 44 × 44 CSS pixels.
- Reduced motion was active in the fresh browser contexts and the CSS removes movement under that preference.
- Every internal link passed. Legal routes, privacy contact, route-specific titles and social metadata, back/forward focus behavior, offline recovery, and service-worker update cleanup passed.
- The unknown route returned HTTP 404 and retained the shared header, navigation, footer, plain “Page not found” heading, and return-home action.
- Lighthouse mobile: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.8 s, CLS 0, total blocking time 100 ms.
- The visual inspection found the documented single-mode survey-sheet design on desktop and phone. The original topographic art has intrinsic dimensions and remains below the mobile image budget.

## Live artifact identity

The documentation head changes only claims/tests/docs after implementation `2428fcb`; there is no later product runtime source change. A build from `/work/repo` matched the live static deployment byte for byte:

| Artifact | SHA-256 |
| --- | --- |
| Root HTML | `0e39eac778969903e94c049d76f4b6ccb1a6eeb4ee00d1c39eb5bf6895bdfcf7` |
| JavaScript | `e90e44b5a0367730a4ff6c75bf5fb6ea16e019a4709bd24ec96bcaa296055bf8` |
| CSS | `5dea072e8c2b9c23588562fb64c4131573281cf737292e11c0bd13597b2769c3` |
| Linux CLI | `23c87d2fa19afedd865c489a1f4aac57a3a8ef934cd339a56fb883733724675c` |

The live API behavior matches implementation `2428fcb`: health, security headers, unauthenticated closure, integration-status flags, original-address rate limiting, and the expected root 404 were observed directly.

## Earlier findings — current disposition

| Earlier finding | Fresh disposition |
| --- | --- |
| Verification 1: false 42-test sample and stale digest | Fixed. The fixture executed 42 tests; the generated and published report evidence matched and verified. |
| Verification 1: unavailable paid checkout | Fixed by honest scope. M2 states $49 per repository per month but exposes no checkout before registration. |
| Verification 1: mobile targets below 44 px | Fixed. Fresh 390 px checks passed on landing, demo, legal, account, billing, and 404 routes. |
| Verification 1: mutable image accepted | Fixed. The live download rejected `ubuntu:latest` before a runtime started. |
| Verification 1: unknown route returned 200 | Fixed. The designed unknown route returns the deliberate HTTP 404. |
| Verification 1: strict Clippy failed | Fixed. Fresh strict Clippy passed. |
| Verification 2: normal checks ran repository tools on the host | Fixed. The executable isolation test captures the locked-down container invocation; the accepted real-Docker matrix remains valid evidence. |
| Verification 2: repository discovery followed directory symlinks | Fixed. Both relative and absolute external-directory symlink regressions passed. |
| Review 1 F-1-1: no-account promise was unlisted | Fixed. One `no-account` entry and one tagged test passed. |
| Review 1 F-1-2: no-tool-install promise was unlisted | Fixed. One `no-tool-install` entry and one tagged test passed. |
| Review 1 F-1-3: no-dependency-install promise was unlisted | Fixed. One `no-dependency-install` entry and one tagged test passed. |
| Review 1 F-1-4: noninteractive-CI promise was unlisted | Fixed. One `noninteractive-ci` entry and one tagged test passed. |
| Review 1 F-1-5: key-permission promise was unlisted | Fixed. One `signing-key-permissions` entry and one tagged test passed. |
| Review 1 F-1-6: “preflight” jargon | Fixed. Current copy consistently uses “repository check.” |
| Review 1 F-1-7: slogan heading | Fixed. The section is “Signed JSON readiness report.” |
| Review 2 F-2-1: routed social metadata stayed on home values | Fixed. Raw and hydrated route metadata passed on every public route. |
| Review 2 F-2-2: unexplained signed-output wording | Fixed. Copy leads with “readiness report” and explains tamper detection. |
| Review 2 F-2-3: undefined image wording | Fixed. Copy explains the exact SHA-256-addressed development image. |
| Verification 5 F-5-1: incomplete, metaphorical 404 | Fixed. The live 404 has the standard structure, plain heading, and recovery action. |
| Verification 5 F-5-2: inconsistent “packet” wording | Fixed. Visitor-facing copy and CLI output use “readiness report.” |

No earlier finding recurred. Verifications 3, 4, and 6 and review 3 had no additional findings.

## Current milestone and external dependencies

M2's product-owned API, SQLite persistence, tenant boundaries, upload validation, privacy controls, account-route states, rate limits, and deployment are accepted by this verification. M3 policy decisions, PR status checks, and readiness-history differences are future scope and were not demanded or presented as working.

The following remain operator dependencies before hosted M2 account and purchase acceptance:

1. **Sociobot Entra CIAM:** register the SPA/API values and complete real first/repeat sign-in, invalid/expired token, and two-account isolation QA.
2. **GitHub App:** register the app and signing key, then complete approved/cancelled installation, state expiry, repository selection/removal, and ownership-conflict QA.
3. **Recurring subscription:** register the Sociobot offer at **$49 per repository per month**, then verify checkout, return, authentic entitlement, renewal, failed renewal, cancellation, expiry, and refund/revocation before exposing purchase.

No checkout, hosted entitlement, working sign-in, or working GitHub installation is claimed. No one-time purchase was substituted.

## Evidence

Screenshots, URL-verifier output, and Lighthouse JSON are under `/work/.evidence/m2qa7/`. The required report copy is `/work/.evidence/qa-report.md`; the machine verdict is `/work/.evidence/qa-result.json`.

## Final result

**PASS — 0 findings, 0 untested claims.**
