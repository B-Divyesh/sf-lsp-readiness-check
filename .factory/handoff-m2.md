# M2 builder handoff

## Status

**Ready for fresh independent M2 QA, with three named external dependencies.** The product-owned private API foundation is deployed and healthy. The accepted M1 CLI and one-click demo remain working. A hosted account, GitHub installation, and paid subscription are not claimed because their registrations and real product QA are still outstanding.

| Record | Value |
| --- | --- |
| Implementation SHA | `2428fcb82bd9af430b8bc98bb1d01421c5660eff` |
| Accepted M1 implementation | `748178140e4f46e75bc596086f09da9bfd3605ba` |
| Static deployment | `e6473d61-9be3-4376-8b48-ac8f6031ed1f` |
| API image | `sociobotregistry.azurecr.io/sf-lsp-readiness-check-api@sha256:51d52ac1d08d5a1ab8c540f45445df3a5dc03daa8fea9fab4874b7f857bf81f3` |
| Product site | <https://lsp-readiness-check.sociobot.in> |
| Product API | <https://lsp-readiness-check-api.sociobot.in> |

The static bundle was deployed before the final API-only rate-limit commit. That commit changed only `server/src/lib.rs` and `tests/api.spec.ts`, so the deployed static bytes are the current site implementation. Documentation commits after the implementation do not require a new product image.

## What shipped in M2

- A Rust/Axum product API with reversible SQLite migrations for users, organizations, memberships, GitHub installations, repositories, policies, readiness runs, and subscriptions.
- Product state at `/data/lsp-readiness-v2.db` on `sf-lsp-readiness-check-api-data`. The app is in single-revision mode with one configured replica. SQLite uses `DELETE` journaling and `unix-dotfile` locks for the SMB-backed mount.
- CIAM JWT validation for RS256, issuer, audience, expiry, and JWKS. Release builds cannot enable test identities.
- Organization-scoped repository, policy, run, export, and delete queries. Two-tenant outcome tests reject repository and policy ID guessing and exclude the other tenant from exports.
- Strict 64 KB signed-report uploads with Ed25519 verification, a deny-unknown-fields schema, length/digest checks, and rejection of source-shaped fields or secret-like evidence.
- A GitHub App connection handoff with short-lived one-time state, server-side App JWT and installation-token exchange, and repository metadata listing. No source permission is required.
- Public health and metrics, structured request logs without request bodies or tokens, request IDs, no-store/security headers, CORS limited to the product origin, and fixed-window per-client/per-organization limits.
- `/sign-in`, `/app`, `/app/repositories`, `/app/repositories/:id/policy`, and `/app/billing` in the existing survey-sheet design. Unregistered providers show a dependency state instead of fake success.
- Owner export/delete controls and one-time repository report-token rotation. The browser account path is outcome-tested against the local release-disabled test identity mode.
- The exact recurring offer: **$49 per repository per month** for private CI checks, policy templates, and readiness history. The offer is unavailable until registration and entitlement QA pass; there is no checkout link.
- Four implemented M2 behavior claims (`tenant-isolation`, `packet-upload-no-source`, `export-delete`, `rate-limit`) plus the honest `subscription-registration-pending` claim. The nine accepted M1 claims remain unchanged and passing.

## Deployment evidence

The API revision `sf-lsp-readiness-check-api--0000005` is healthy with one active replica, a minimum and maximum of one, and the immutable image above. `GET /healthz` returned:

```json
{"database":"ok","schema_version":1,"status":"ok"}
```

A product revision restart returned the same schema version, then settled back to one healthy replica. This proves the deployed schema survives a restart. Tenant-row restart persistence and backup/restore are tested against fresh SQLite files locally; a hosted tenant-row check cannot be performed honestly until CIAM is registered.

A real hosted burst to the protected API returned HTTP `429` with `Retry-After: 58`. The first live attempt exposed rotating ingress peer addresses; implementation `2428fcb` fixed the cause by using the ingress-provided original client address, and its proxy-address regression passes locally.

`GET /api/v1/config` reports all three integrations as false. `/api/v1/session` without a token returns `401`. The empty API root deliberately returns `404`; `/healthz` is the service probe.

The deployment wrapper completed the image, app update, DNS, and certificate work. It was stopped only after it kept polling the deliberate API-root `404`; direct `/healthz` and revision checks then passed.

## Verification completed

From a new clone of `2428fcb`:

- `npm ci`: 23 packages, zero vulnerabilities.
- Every one of the 14 commands in `.factory/claims.json`: passed separately.
- `npm test`: 4 API unit tests, 11 CLI/library tests, and 38 Playwright tests passed.
- `npm run build`: wrote `dist/site`; initial JavaScript was 30.45 KB (9.54 KB gzip) and CSS was 14.39 KB (4.11 KB gzip).
- `cargo fmt --all --check`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- `npm audit --audit-level=high`: zero vulnerabilities.
- `cargo package -p lsp-readiness-check --locked --allow-dirty`: packaged and verified the 0.1.2 CLI. `--allow-dirty` is needed after `npm ci` because Cargo enumerates generated `node_modules` files in this combined workspace.
- Clean consumer prefix: installed the packaged CLI; `--help`, `--version`, `demo`, generated-report `verify --json`, and missing-report recovery passed.

Live checks:

- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test`: 32 passed; 6 local-only authenticated API tests skipped by design.
- Fresh 1440×900 and 390×844 browser contexts stated the job, audience, and sample action before scrolling. The demo showed both language servers, two formatters, 42 tests, and its Ed25519 result.
- The demo banner persisted. Reset recreated only `demo:lsp-readiness-check`; a real-data sentinel was unchanged. “Start for real” removed only the demo key.
- Phone width was 390 CSS pixels with no horizontal overflow. Reduced-motion preference was active. Browser console errors: zero.
- The worker URL verifier returned HTTP 200, `lang=en`, one H1, a main landmark, no missing image alt text, no unlabeled buttons, and zero console errors.
- Playwright Axe checks found no serious or critical issue on every public/account route and the designed 404.
- Lighthouse mobile: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.71 s, CLS 0.0007, total blocking time 0 ms.
- Offline demo reload, old service-worker cache cleanup, keyboard/focus navigation, 44 px mobile targets, internal links, route titles/social metadata, legal pages, and deliberate HTTP 404 recovery all passed in the suite.

Evidence is under `/work/.evidence/m2/`. Required public copies are `/work/.evidence/catalog-description.txt` and `/work/.evidence/billing-offer.json`.

## Operator dependencies before hosted acceptance

1. **Sociobot Entra CIAM:** register the product SPA/API values, add the seven `CIAM_*` settings through the operator secret path, then test real first/repeat sign-in, expired/wrong tokens, and two-account isolation.
2. **GitHub App:** register the exact callback, metadata-only repository permission, app identity and signing key, then test approved/cancelled installation, state expiry, repository selection/removal, and installation ownership conflicts.
3. **Sociobot recurring subscription:** register USD $49/repository/month with the product return URL and provide the recurring entitlement/webhook contract. Test checkout, return, authenticity, renewal, failed renewal, cancellation, expiry, and refund/revocation before exposing a buy action.

Exact non-secret values and acceptance paths are in `.factory/m2-operator-dependencies.md`. No one-time purchase was substituted, no provider credential was invented, and no paid deliverable was removed.

## Known gaps and next step

- Hosted CIAM sign-in, a real GitHub installation, and the subscription lifecycle are unavailable and untested. They are external dependencies, not shipped behavior.
- The product API foundation is live, but authenticated hosted tenant operations cannot receive real product QA until CIAM is configured.
- GitHub webhook validation, PR check status, policy decisions, and readiness-history differences remain M3. Do not start M3 until M2 receives independent acceptance.
- The real Docker readiness matrix remains the independently accepted M1 evidence. It was not repeated or downgraded in this milestone.

## Reproduce

```sh
npm ci
npm test
npm run build
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo package -p lsp-readiness-check --locked --allow-dirty
PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test
```
