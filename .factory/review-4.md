# Strict review 4 — M2 private CI foundation

- **Date:** 2026-09-06
- **Live URL:** <https://lsp-readiness-check.sociobot.in>
- **Current milestone:** M2 — product-owned private CI foundation
- **Implementation candidate:** `2428fcb82bd9af430b8bc98bb1d01421c5660eff`
- **Documentation/test head:** `a536fcd620ae9a56b1a018b743e20aced12478d4`
- **Verdict:** **PASS**
- **Findings:** **0**
- **Untested claims:** **0**

## Verdict

**PASS — 0 findings, 0 untested claims.** The deployed M2 product-owned API foundation, accepted M1 CLI, static site, and isolated demo meet the current milestone's public promises. Hosted CIAM sign-in, GitHub App installation, and recurring subscription behavior remain unavailable external dependencies and are not claimed as working.

## Scope and candidate identity

M1 remains accepted at `748178140e4f46e75bc596086f09da9bfd3605ba`. This review examined M2 implementation `2428fcb`; the later head changes only claims/tests, README, and factory records. A fresh build at documentation head matched the live root HTML, hashed JavaScript, and CSS byte-for-byte. The live downloaded CLI reported version `0.1.2` and matched the reviewed public behavior.

M3 policy decisions, GitHub PR status checks, and readiness-history differences are planned work. They were not required for M2 and are not represented as operating features.

## First screen and demo sandbox

Fresh 1440 × 900 desktop and 390 × 844 phone Chromium contexts started at scroll position zero.

| Review question | Observed live answer | Result |
| --- | --- | --- |
| Job | “Verify tooling before an agent edits” | Pass |
| Audience | Teams onboarding contributors who need navigation, diagnostics, formatting, and tests ready | Pass |
| First action | “Try it with sample data” — “See a finished probe in one click.” | Pass |

The action was visible in both first viewports. It entered `/?demo=1` and immediately showed an opinionated populated result: 5/5 required checks, TypeScript and Rust language servers, Prettier, Rustfmt, 42 passing tests, and Ed25519 tamper detection. The persistent “Demo — sample data, nothing is saved” banner remained visible.

Reset recreated only `demo:lsp-readiness-check`; **Start for real** removed only that demo key. A separately inserted real-data sentinel remained unchanged through both actions. Desktop and phone had no horizontal overflow or console/page error. The live suite also passed offline reload, service-worker cache replacement, keyboard/focus navigation, reduced motion, accessibility, and the designed HTTP 404 recovery.

## Claims and clean checkout

A new local clone at `a536fcd` ran `npm ci` successfully (23 packages; zero high-severity audit findings). Every command declared in `.factory/claims.json` was run exactly as written and passed:

| Claim IDs | Result |
| --- | --- |
| `sample-probe`, `local-operation`, `signed-packet`, `offline-demo`, `no-account` | Pass |
| `no-tool-install`, `no-dependency-install`, `noninteractive-ci`, `signing-key-permissions` | Pass |
| `tenant-isolation`, `packet-upload-no-source`, `export-delete`, `rate-limit` | Pass |
| `subscription-registration-pending` | Pass |

`npm test` passed: 4 API unit tests, 11 CLI/library tests, and 38 Playwright tests. `npm run build`, `cargo fmt --all --check`, strict `cargo clippy --workspace --all-targets --all-features -- -D warnings`, `npm audit --audit-level=high`, and `cargo package -p lsp-readiness-check --locked --allow-dirty` all passed. The build wrote `dist/site/` with 30.45 KB JS (9.54 KB gzip) and 14.39 KB CSS (4.11 KB gzip).

The packaged `0.1.2` CLI installed into a separate consumer prefix. Its help, version, demo, and `verify --json` flow worked. The live downloadable Linux binary likewise generated and verified a report; `check --image ubuntu:latest` failed closed with exit code 2 before a runtime could start.

## Live API, privacy, and availability

- `GET /healthz` returned HTTP 200 with `database: ok` and schema version 1.
- `GET /api/v1/config` accurately reported CIAM, GitHub App, and subscription as unconfigured.
- `GET /api/v1/session` without a token returned HTTP 401 with an actionable recovery message.
- A fresh live public-config burst reached HTTP 429 on attempt 9 with `Retry-After: 28` and the expected rate-limit response.
- The API root's HTTP 404 is deliberate; `/healthz` is the health endpoint.
- CORS named only the product origin; response headers included `nosniff`, no-referrer policy, no-store, and a restrictive API CSP.
- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` passed **32** tests and correctly skipped **6** local test-identity API tests.

The public claim audit found no unlisted visitor-reliance promise. The product does not use analytics, third-party scripts, or remote fonts. The demo made no cross-origin request. The $49 per repository per month offer is correctly labeled unavailable, exposes no checkout, and does not substitute a one-time purchase.

## Earlier findings

All earlier findings were checked for recurrence. The prior missing claim tests, “preflight”/slogan wording, route metadata, cryptographic/image terminology, mobile target size, deliberate 404 structure, and “packet” terminology defects remain fixed. The previously accepted real-Docker matrix is preserved as M1 evidence and was not incorrectly downgraded for lack of new runtime proof.

## Current milestone and external dependencies

M2's product-owned service work is evidenced: SQLite persistence/migration tests, organization-scoped API tests, strict signed-report validation, no-source rejection, export/delete, rate limits, health, account-route dependency states, and the deployed public API behavior.

The following are **operator dependencies**, not working hosted capabilities:

1. **Sociobot Entra CIAM:** register the product integration, then complete real first/repeat sign-in, expired/wrong-token, and two-account isolation QA.
2. **GitHub App:** register app identity/callback/signing key, then complete installation approval/cancellation, state expiry, repository selection/removal, and ownership-conflict QA.
3. **Recurring subscription:** register the Sociobot **$49 per repository/month** product and entitlement contract, then complete checkout, return, renewal, failed renewal, cancellation, expiry, refund/revocation, and reconciliation QA.

No hosted sign-in, GitHub installation, checkout, entitlement, or one-time-purchase behavior is claimed by this PASS.

## Evidence

Review logs and browser captures are under `/work/.evidence/review-4/`. The required report copy is `/work/.evidence/qa-report.md`; the corresponding machine result is `/work/.evidence/qa-result.json`.

## Final result

**PASS — 0 findings, 0 untested claims.**
