# Venture plan — LSP Readiness Check

**Plan date:** 2026-09-05

**Current milestone:** M1 — **accepted** on 2026-09-05. The deployed implementation is `b3714c16ec78b14d5d403d7eaa98e5ac0b27ee02`; later documentation commits are recorded separately in the handoff.
**Next milestone:** M2 — authenticated private-CI foundation and subscription (planned; not started).

## 1. Product contract

### Customer and situation

Small product teams bring contributors into repositories where agents may edit code. Before allowing an agent to start, they need a concrete answer to: “Can this repository’s language navigation, diagnostics, formatting, and tests actually run in the selected environment?” Today they read setup notes, install tools one by one, or accept edits made without usable semantic tooling.

### Promise

Produce a local, signed readiness report from a repository’s selected development image before an agent edits it.

### The three jobs

1. **Prove current readiness.** Detect supported languages, initialize available language servers, check formatters, run a declared test command, and produce a verifiable capability packet.
2. **Apply a repository policy in CI.** Let a repository owner state which checks are required and make a pull request fail clearly when the packet does not meet that policy.
3. **See readiness changes over time.** Let an authorized team see which readiness check changed between runs without uploading repository source.

### Wedge and evidence

The opportunity is a release gate for usable semantic tooling, rather than another extension installer or development container. The researched demand is recorded in [brief.json](brief.json): the Codex request for automatic LSP support had 566 reactions, while dev containers and IDE extensions do not prove repo-specific navigation, diagnostics, formatting, and tests work together. The pilot success measure remains 80% of onboarding PRs passing the contract before agent edits and a 30% reduction in environment-related review failures.

### Monetisation and deliberate limits

The current public product is free: a Rust CLI, a bundled sample demo, and static documentation. It has **no account, API, paid plan, checkout, history service, GitHub App, or sign-in**.

The researched future offer is **$49/repository/month** for private CI checks, policy templates, and readiness history. It is planned for M2; it must not be displayed or described as available before a real subscription integration is registered, tested, and accepted.

Out of scope throughout M1–M3: hosting language servers, installing or upgrading tools/dependencies, an IDE, autonomous dependency upgrades, uploading source code, AI features, and direct payment-provider integration.

## 2. Current implementation and honest status

### What is implemented and accepted at the functional-test level

- `lsp-readiness demo` probes the shipped `examples/northstar-api` fixture through the inspection engine, records TypeScript and Rust LSP capabilities, two formatters, 42 fixture tests, and an Ed25519-signed JSON packet. `verify --json` validates the packet.
- `lsp-readiness check`/`container` require a SHA-256-pinned image and build a Docker/Podman invocation with no network, read-only root/source, dropped capabilities, no-new-privileges, and temporary work paths. The host creates and retains the signing key.
- The static site provides a one-click isolated demo (`/?demo=1` or `/demo`), a persistent reset/exit banner, local `demo:lsp-readiness-check` storage, offline reload after the first visit, privacy/terms pages, a designed 404, and no cross-origin demo request.
- `/`, `/demo`, `/privacy`, and `/terms` are prerendered with their own title, description, canonical, Open Graph, and Twitter metadata before JavaScript. Client navigation updates the same metadata.
- The nine public claims are present in [claims.json](claims.json) and have tagged browser/CLI tests. The functional, isolation-argument, accessibility, mobile, route, and package checks are in [tests/site.spec.ts](../tests/site.spec.ts) and [tests/cli_isolation.rs](../tests/cli_isolation.rs).

On 2026-09-05, a fresh clone at the deployed implementation ran all nine exact claim commands, `npm test` (**11 Rust tests and 25 Playwright tests passed**), `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo package --allow-dirty`, and a fresh `cargo install` consumer demo/verify. The live suite also passed: `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` (**25/25**). `verify-url.sh` reported HTTP 200, no console errors, `lang=en`, one H1, a main landmark, no missing image alt text or unlabeled buttons, and a 745 ms load in this run.

### What is demonstrated, not yet proven end to end

| Area | Evidence | Limit of the evidence |
| --- | --- | --- |
| Disposable-container boundary | Fake-runtime tests assert the exact Docker/Podman flags, source mount, pinned-image validation, host signing, command traps, and source immutability. A real Docker matrix on the product-named private helper passed ready, non-ready, LSP-timeout, and runtime-error outcomes against digest-pinned images; all source checksums were unchanged and valid packets verified. | Docker is validated once against controlled Ubuntu 24.04 test images. Podman and arbitrary customer images remain customer-environment compatibility work; the image must be able to run the installed CLI binary. |
| Bundled demo | The fixture executes and the packet verifies locally; the browser replay uses shipped sample data. | It proves only the trusted `northstar-api` sample. It does not prove an arbitrary customer repository, container image, test command, or policy. |
| Signed packet | Packet integrity and tamper rejection are tested. | The product has no policy service or identity binding that says which team/key/repository may rely on a packet. |
| Privacy boundary | Current CLI has no network client and the browser demo made same-origin requests only. | This is the current no-server product only. It does not establish the future account/history data boundary. |
| Prior review verification | [review-2.md](review-2.md) was the current release gate and recorded FAIL. | Its three copy/metadata findings are repaired and reverified below; its earlier findings remain covered by the current claim and browser suites. |

### M1 repair and acceptance record

The former blockers were rechecked as observable outcomes; none is merely marked fixed.

1. **F-2-1 — fixed.** The static build emits per-route HTML under `/demo`, `/privacy`, and `/terms`; static-host routing serves it. The new browser regression reads raw response metadata before JavaScript and checks it again after hydration. Live direct GETs returned the correct title, description, canonical, Open Graph, and Twitter values.
2. **F-2-2 — fixed.** Landing copy now leads with “Signed JSON readiness report,” explains that its signature makes tampering detectable, and leaves Ed25519 as the implementation detail. The visible browser assertion and copy audit cover the wording.
3. **F-2-3 — fixed.** Landing and README say that the user chooses the exact development image and explain the SHA-256 address as a way to run the same tools each time.
4. **Operational validation — fixed.** The private helper ran `lsp-readiness check` through Docker against controlled digest-pinned images: ready exited 0 with a verified packet; missing tools exited 1 with a verified non-ready packet; a sleeping LSP exited 1 with the five-second timeout evidence and a verified packet; a BusyBox runtime mismatch exited 2 with an actionable container error. Each source checksum remained unchanged.

**Result:** the latest review findings and the real-engine requirement are satisfied. M1 is accepted; M2 remains planned only.

## 3. M1–M3 delivery contract

### M1 — free local readiness proof, demo, and release repairs

**Status:** accepted on 2026-09-05.

**Routes and commands:** `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, `/404`; `lsp-readiness check`, `container`, `demo`, and `verify`.

**Scope:** preserve the current CLI and static demo. Repair F-2-1 through F-2-3. Add static metadata coverage and plain-language copy regressions. Execute the real container-engine smoke against a team-owned, digest-pinned test image; do not embed or report its credentials.

**Definition of done:**

- Every direct route serves its own accurate title, description, canonical, Open Graph, and Twitter metadata before JavaScript; the hydrated route matches it.
- The cold landing explains the signed report and exact-image isolation in plain language, while the underlying Ed25519 and SHA-256 behaviour remains tested.
- All current [claims.json](claims.json) commands pass from a clean checkout, `npm test` passes, the site build writes `dist/site/`, and `cargo fmt`, strict Clippy, and `cargo package` pass.
- The published normal-command smoke passes with a real engine and pinned image, or reports a bounded non-ready/runtime failure without mutating the source. The test record identifies only a non-sensitive image digest/reference as appropriate.
- Live `/`, demo, legal routes, 404, accessibility, mobile, keyboard, reduced-motion, same-origin browser traffic, and offline demo are reverified. The review verdict becomes PASS.

**Current claims/tests:**

| Claim ID | Observable test |
| --- | --- |
| `sample-probe` | The bundled fixture runs 42 tests and renders its signed readiness result. |
| `local-operation` | Browser demo requests stay same-origin; fake-runtime contract asserts no-network/locked-down invocation and symlink regressions. |
| `signed-packet` | CLI demo packet verifies with Ed25519. |
| `offline-demo` | A new browser context reloads the demo offline after first visit. |
| `no-account` | Browser and CLI demo work without credential/auth requests. |
| `no-tool-install` / `no-dependency-install` | Command traps are not run and source remains unchanged. |
| `noninteractive-ci` | Public CLI command paths finish with stdin closed. |
| `signing-key-permissions` | First normal check writes a Linux mode-0600 key. |

The exact commands are the entries in [claims.json](claims.json), all of the form `npm test -- --grep @claim:<id>`.

### M2 — authenticated private CI foundation and subscription

**Status:** planned; not started. It does not become current until M1 is accepted.

**Scope:** add the minimal authenticated service required for a private repository to send a signed, source-free readiness result, configure the GitHub App/CI handoff, persist tenant-scoped policy and run metadata, and sell the stated subscription only through the Sociobot billing service.

**Routes/screens:** `/sign-in`, `/app`, `/app/repositories`, `/app/repositories/:id/policy`, `/app/billing`, plus authenticated API endpoints. The static public routes and anonymous demo remain available and separate.

**Definition of done:**

- A real Sociobot Entra CIAM sign-in creates a user and organization. Every repository/policy/run query is authorized by organization membership; cross-tenant ID guessing is rejected in API and browser tests.
- A GitHub App installation can connect an authorized repository. The customer’s own CI executor runs the CLI; the service receives only an explicit, schema-validated capability packet and PR/repository identifiers needed for the check. It never receives a checkout or source file contents.
- SQLite on `/data` stores tenant-scoped account, repository, policy, run, and subscription state. Migrations, backup/restore, export/delete, rate limits, structured logs, health, and a no-source request-body test exist before a production persistence claim.
- The $49/repository/month subscription uses a factory-registered Sociobot subscription integration. Checkout/return/webhook verification and entitlement reconciliation pass in the provider’s authorized test mode. Until that exists, there is no paid button or “subscription available” copy.
- The demo is still one click and cannot read/write production tenant data. It uses a separate ephemeral/demo namespace and makes no billing spend.

**New claims to add only with implementation:** `tenant-isolation`, `packet-upload-no-source`, `private-ci-check`, `subscription-entitlement`, `export-delete`, and `rate-limit`. Each must have a recorded-fixture or approved test-mode integration test; a mock checkout alone is not billing proof.

### M3 — repository policies, PR gate, and readiness history

**Status:** planned; not started.

**Scope:** deliver jobs two and three over the M2 foundation: editable policy templates, CI/PR readiness decisions, and a tenant-scoped history that shows readiness changes.

**Routes/screens:** `/app/repositories/:id/policy`, `/app/repositories/:id/runs`, `/app/runs/:runId`, and an authenticated installation/setup screen with a copyable CI snippet. No source viewer is added.

**Definition of done:**

- An owner can create a policy defining required language-server, formatter, test, and packet-signature conditions. Validation returns an actionable pass/fail result from a real uploaded packet; the free local CLI remains useful without an account.
- An authorized PR run receives a GitHub check/status whose conclusion and detail match the stored policy decision. Webhook signatures, delivery replay handling, installation authorization, and retry/error states are tested with signed fixtures plus an authorized integration environment.
- History retains the allowed packet metadata and policy decision, shows the exact changed capabilities between two runs, and is constrained to the owning tenant. Export/delete and retention behaviour are tested.
- M2’s billing entitlement is enforced server-side for private policy/history features, but never for the free CLI, accessibility, packet verification, or core export.

**New claims to add only with implementation:** `policy-gate`, `github-check-status`, `history-diff`, `history-tenant-isolation`, and `subscription-enforcement`.

## 4. Architecture and data boundaries

### Current architecture (implemented)

```text
User machine / customer CI
  Rust `lsp-readiness` CLI
    -> selected Docker or Podman image (network disabled; read-only source; tmpfs copy)
    -> host-signed JSON capability packet

Static site at lsp-readiness-check.sociobot.in
  Vite/TypeScript -> bundled demo/replay + service worker
  localStorage key: demo:lsp-readiness-check
  no product API, analytics, account, billing, or source upload
```

The Rust CLI is intentionally low-dependency and compiled as one binary. `src/lib.rs` holds supported-language detection, LSP initialize probing, formatter/test execution, inventory digesting, and Ed25519 signing. `src/main.rs` owns the CLI boundary and container invocation. The site is Vite/TypeScript with self-hosted assets and a service worker; it is not a backend.

### M2/M3 architecture (planned, not present)

```text
Customer-controlled CI -> local CLI -> explicit capability packet upload
                                    -> Rust/Axum product API
                                         -> SQLite at /data/lsp-readiness.db
                                         -> Sociobot Entra CIAM (identity only)
                                         -> GitHub App APIs/webhooks (installation and PR status)
                                         -> Sociobot billing API (subscription entitlement only)

Vite static public site and demo remain separate from authenticated app/API.
```

Use a boring Rust/Axum API only when M2 begins. Pin a single replica with its product-specific `/data` mount; no shared PostgreSQL is available or needed. Use structured JSON logs with request IDs and redaction, a health endpoint, bounded request sizes, per-IP and per-tenant rate limits, and migrations tested against a copy of a fixture database. No background task may fetch source; webhook retry and entitlement reconciliation jobs operate only on allowed metadata.

### Data model and ownership (planned)

| Entity | Owner/scope | Allowed fields | Prohibited fields |
| --- | --- | --- | --- |
| User | CIAM subject | subject ID, display name/email only where CIAM returns it | passwords or identity-provider tokens in SQLite |
| Organization / membership | organization tenant | opaque IDs, roles, membership timestamps | cross-tenant membership inference |
| GitHub installation / repository | one organization | installation/repository IDs, display name, authorized metadata | repository checkout, source files, long-lived token in client |
| Policy | one repository | required checks, version, timestamps | arbitrary shell commands supplied to server |
| Readiness run / packet | one repository and policy version | packet schema/version, source inventory digest, language/capability statuses, command labels/evidence, signature/public key, PR/run IDs, timestamps | source contents, git diff, raw test logs, secrets |
| Subscription entitlement | one organization/repository | provider product/customer/subscription references, status, expiry, verified-at | card details or payment-provider credentials |
| Audit/security event | organization/system | actor, action, target opaque ID, timestamp, outcome | source or token values |

Every persistence query takes `organization_id` from verified server-side identity and includes it in its predicate. Primary-key lookup alone is forbidden. Packet upload uses a strict schema, request-size cap, field allowlist, secret-marker test, and rejects attachments/raw logs. Capability evidence needs a length limit and redaction policy before it is persisted.

## 5. External dependencies and boundaries

| Dependency | Current status | Needed by | What must be true before it is claimed |
| --- | --- | --- | --- |
| Docker or Podman plus a customer/team digest-pinned Linux x86-64 development image | Required for normal `check`; Docker validated on a controlled private helper | M1 | Customer images must contain the requested tools and a glibc version compatible with the installed CLI binary. Podman remains a customer-environment dependency, not a claimed M1 validation. |
| Customer repository tools/tests | Customer-controlled | M1+ | The selected image contains tools/dependencies; the CLI must not install them or mutate source. |
| Static-hosting deployment | Present at the live URL | M1 | Repaired route metadata must be deployed and independently reviewed. |
| Sociobot Entra CIAM configuration | Not present in this product | M2 | Factory provisions the product integration; authenticated flows and tenant authorization tests pass. |
| GitHub App registration, webhook endpoint, and authorized customer installation | Not present | M2/M3 | Factory/customer authorizes the app; server-only credentials, signature verification, and installation authorization are tested. |
| Sociobot billing subscription registration/API contract | Not present; the supplied paid-unlock guide is one-time-license guidance, not proof of subscriptions | M2 | Factory registers the product and supplies the approved subscription contract/test mode. The product uses only Sociobot endpoints, never direct Dodo/payment credentials. |
| Product API hosting and `/data` SQLite mount | Not needed today; absent | M2 | Fleet creates the product-specific service/mount and backup/restore is verified. |

No external dependency is an implemented capability today. Billing, messaging, HMRC access, sign-in, GitHub App access, and server persistence are not available in the current product and are not requested from a product worker.

## 6. Risks, experiments, and sequencing

| Risk/unknown | Retirement experiment | Decision boundary |
| --- | --- | --- |
| Locked-down runtime arguments differ across Docker/Podman or break common development images | Docker passed the ready/non-ready/timeout/runtime-error matrix on controlled digest-pinned images. Run the same matrix on a customer Podman environment before claiming Podman runtime validation. | Docker normal checks are end-to-end validated; Podman remains unvalidated. |
| Packet capability evidence can contain more data than intended | Use a sentinel in a fixture source/test output; assert upload allowlist excludes it and reject oversized evidence. | No M2 persistence until the test passes. |
| CIAM/GitHub installation cannot provide safe repository-level authorization | Test two organizations, two installations, forged/replayed webhook, and ID guessing. | No private repository/history claim until all reject correctly. |
| The $49 price cannot be provisioned through the allowed billing path | Obtain an approved Sociobot subscription test contract and run a test entitlement lifecycle. | Keep paid copy absent; do not substitute a one-time unlock or direct provider checkout. |
| A repository owner will not trust a report without source upload | Pilot the local packet + CI artifact flow with 3–5 team repositories and measure successful onboarding PRs. | Revisit packet fields/policy UX only after privacy and isolation tests hold. |

**Sequence:** M1 accepted → M2 identity, tenant storage, opt-in packet upload, and subscription → independent M2 PASS → M3 policy gate/history. Future capability is never a current public promise.
