# Handoff: M2 verification 7

Independent QA passed the deployed M2 product-owned foundation with **0 findings and 0 untested claims**. The reviewed runtime implementation is `2428fcb82bd9af430b8bc98bb1d01421c5660eff`; verification-only changes are at `a011cc3e31863c8d56b01ce1b24701919bb3af7e`; the documentation reviewed began at `8da11e5a107bdfffa18489164266931dc17605bc`.

From a fresh checkout, all 14 exact claim commands passed. `npm test` passed 4 API unit tests, 11 CLI/library tests, and 38 Playwright tests. Build, type checks, formatting, strict Clippy, npm audit, Cargo packaging, a clean consumer install, and the live downloadable CLI passed. The live suite passed 32 tests with 6 correctly skipped local-auth tests.

Fresh desktop and phone checks passed the first-screen, one-click sample, persistent demo label, reset/exit isolation, keyboard, touch-target, reduced-motion, route, legal, offline/update, accessibility, privacy, and designed-404 requirements. Lighthouse mobile scored 99 performance and 100 for accessibility, best practices, and SEO. Local process restart preserved a tenant row in SQLite; the live API returned healthy schema version 1 and a real 429 with `Retry-After`. The live static assets and Linux CLI matched the local candidate build byte for byte.

Hosted CIAM sign-in, a real GitHub App installation, and the **$49 per repository per month** recurring subscription remain separate operator dependencies. They are shown as unavailable and are not claimed as working; no checkout or hosted entitlement is exposed. M3 policy decisions, PR checks, and history differences remain future scope.

Full evidence and reproduction details are in [verification-7.md](verification-7.md). Builder and deployment details remain in [handoff-m2.md](handoff-m2.md).
