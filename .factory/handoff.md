# Handoff: M2 builder

The product-owned M2 foundation is deployed and ready for independent QA at implementation `2428fcb82bd9af430b8bc98bb1d01421c5660eff`. The accepted M1 CLI and demo remain intact.

The live API is healthy on one configured replica with SQLite on the durable product `/data` mount. All 14 claim commands and the full clean-clone suite pass. Live desktop, phone, accessibility, offline, route, 404, rate-limit, and restart checks pass. Lighthouse mobile scored 99 performance and 100 for accessibility, best practices, and SEO.

Hosted CIAM sign-in, a real GitHub App installation, and the $49/repository/month recurring subscription are not available yet. They remain named operator dependencies and require real hosted QA; there is no checkout or entitlement claim.

See [handoff-m2.md](handoff-m2.md) for implementation, deployment, verification, dependency, and evidence details.
