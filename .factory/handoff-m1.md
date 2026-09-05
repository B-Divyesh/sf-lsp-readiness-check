# M1 handoff — LSP Readiness Check

## Outcome

M1 is accepted. It delivers a free Rust CLI that checks a repository inside a digest-pinned locked-down container and writes a signed JSON readiness report, plus a one-click static demo of the bundled `northstar-api` sample.

The final repair fixed verification-5:

- The HTTP 404 response now uses the site header, navigation, skip link, main landmark, footer, plain Page not found heading, and return-home action.
- Privacy, demo, README, CLI help, and CLI completion output consistently call the signed output a readiness report.

## Build and live evidence

- Implementation: `748178140e4f46e75bc596086f09da9bfd3605ba`.
- Deployed documentation baseline: `01102b7be63059becb95b13f47222ebfc274270a`.
- Deployment: Static Web App `fb33c0f7-9af2-428f-969a-8a41f8f7373e`.
- Clean clone: `npm ci`, all nine declared claims, `npm test` (11 Rust + 27 Playwright), build, strict Clippy, package, and fresh consumer install passed.
- Live: `verify-url.sh` passed; live Playwright passed 27/27; cold desktop and phone flows, demo reset isolation, offline demo, accessibility, and the intentional 404 passed.
- Lighthouse: 99 performance, 100 accessibility, 100 best practices, 100 SEO.

The live root HTML, static 404, JS, CSS, and downloadable CLI match the local build exactly. Full artifact hashes and command evidence are in [handoff.md](handoff.md).

## Limits and next milestone

M1 has no backend, account, persistent service data, billing, checkout, or GitHub App. Its Static Web App has no `/data` mount, so service persistence, health, tenant isolation, rate limits, and billing checks are not applicable.

Existing real-Docker evidence covers the unchanged locked-down command path. Podman compatibility remains a separate customer-environment dependency. M2 is planned, not started: it needs Entra CIAM, GitHub App, Sociobot subscription registration, `/data` SQLite, tenant isolation, and entitlement checks.
