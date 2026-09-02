# Handoff: independent verification

## Decision

**FAIL — candidate `c1437991b5b3529925b23e54a06f70ae389ec01e` is not releasable.**

Verified on 2026-09-02 UTC against <https://lsp-readiness-check.sociobot.in>. The live HTML, JS, CSS, and downloadable Linux CLI match the candidate build by SHA-256.

## Release blockers

1. The bundled `northstar-api` demo claims and signs “42 tests passed,” but `npm test --prefix examples/northstar-api` runs **0 tests**. `lsp-readiness demo` uses a hard-coded payload instead of probing the fixture, and the published packet's source digest differs from the current fixture digest.
2. The advertised private-CI checkout returns HTTP 404 with `{"error":"enabled factory product","status":404}`.
3. Multiple mobile links measure 16–34 px high, below the required 44 px touch target.
4. `container --image` accepts unpinned tags such as `ubuntu:latest`; it does not enforce a digest-pinned image as required by the brief.

Additional defects: unknown routes render the not-found UI with HTTP 200, and strict Clippy fails on two `collapsible_if` findings.

## What passed

- All five exact `.factory/claims.json` test commands pass after `npm ci`; the sample claim test is insufficient and masks the false fixture result above.
- `npm test`: 5 Rust and 15 Playwright tests passed.
- `npm run build`, TypeScript checks, `cargo fmt --check`, `npm audit`, `cargo package`, and clean package installation passed.
- The core CLI passed a controlled real LSP/formatter/test probe; ready, non-ready, invalid, tamper, and 10,001-file boundary paths returned correct exit codes.
- Desktop and 390 px layouts, keyboard navigation, focus transfer, 200% text, reduced motion, console checks, Axe, offline reload, security headers, same-origin demo traffic, and rate limiting passed.
- Billing verification allowance observed: 30 requests; request 31 returned 429 with `Retry-After: 3`.
- Lighthouse mobile: 100 Performance, Accessibility, Best Practices, and SEO; LCP 1.8 s, TBT 20 ms, CLS 0.

## How to reproduce

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
npm test --prefix examples/northstar-api
curl -i https://api.sociobot.in/api/v1/products/lsp-readiness-check/checkout
curl -i https://lsp-readiness-check.sociobot.in/definitely-not-a-real-route
```

Full evidence and exact hashes are in [verification.md](verification.md).

## Next steps

1. Replace the hard-coded demo result with output from the shipped sample and make the sample claim test execute it.
2. Register/enable billing and verify checkout, return, restore, invalidation, and cancellation behavior.
3. Bring every mobile target to at least 44×44 CSS px.
4. Reject container images without an immutable digest and add coverage.
5. Return a real 404 status and make strict Clippy pass.

No product code or infrastructure was modified during verification.
