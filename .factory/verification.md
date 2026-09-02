# Independent verification — FAIL

- Work order: `lsp-readiness-check-verify-1`
- Candidate: `c1437991b5b3529925b23e54a06f70ae389ec01e`
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Verified: 2026-09-02 UTC
- Result: **FAIL — do not release this candidate**

The candidate builds, its core CLI can produce and verify a real readiness packet, and the live static deployment is byte-for-byte consistent with the candidate. Release is blocked by a false bundled-demo result and a broken paid checkout. Accessibility and contract issues remain as well.

## Release-blocking findings

### High — the bundled demo signs and displays test evidence that the sample cannot produce

Every demo surface says the bundled `northstar-api` sample has **42 passing tests**:

- `lsp-readiness demo`
- `/demo`
- the landing terminal
- `site/public/sample/northstar-api.lsp-readiness.json`
- `.factory/demo.md`

Fresh evidence from the shipped sample:

```text
$ npm test --prefix examples/northstar-api
TAP version 13
1..0
# tests 0
# pass 0
# fail 0
```

The sample contains only `package.json`, `src/index.ts`, and `src/lib.rs`; it contains no tests. The CLI `demo` command calls a hard-coded `demo_payload()` and never probes `examples/northstar-api`. The published sample packet also does not describe the current fixture:

```text
current fixture digest:   sha256:d3ce70325d489ac4f0cba1284482ff044e008335d18ed3e6169bf84b1578eab2
published packet digest:  sha256:1df019b6566159a4b012f1d68133f30aa80b16ec81ce1785a93a555a07b93e2d
```

The `@claim:sample-probe` test passes because it asserts that the hard-coded words “42 tests passed” are visible. It does not run the bundled sample or compare the signed packet to it. This violates the claims and demo-sandbox contracts and is especially damaging for a product whose job is to provide trustworthy signed evidence.

Required fix: make the bundled fixture produce the stated result through the real probe, generate the demo packet from that run, and make the claim test assert the fixture's actual output and digest. Alternatively, state the honest result everywhere.

### High — the advertised paid checkout is not deployed

The first-party “Buy private CI” link points to the required Sociobot route, but that live route fails:

```text
GET https://api.sociobot.in/api/v1/products/lsp-readiness-check/checkout
HTTP/2 404
{"error":"enabled factory product","status":404}
```

Visitors are promised private CI for `$49 per repository each month`, but cannot begin checkout. The previous handoff's registration gap is still present in fresh live evidence.

Required fix: register and enable this product in the Sociobot billing service, then verify the complete hosted checkout/return/license flow before release. If the service is not ready, remove the purchase action and do not present the plan as purchasable.

### Medium — several mobile interactive targets are below 44 px

At a 390×844 CSS-pixel viewport, measured heights include:

- wordmark: 34 px
- header `Demo` and `Privacy` links: 24 px
- `Download Linux binary`: 26 px
- inline terms link: 16 px
- footer links: 22 px

The primary buttons meet the target size, but the links above do not meet the non-negotiable 44×44 px touch-target baseline.

### Medium — the container path does not enforce a pinned image

The help and site describe a “pinned development image,” and the brief requires pinned, provenance-verifiable inputs. `--image ubuntu:latest` is accepted and processing proceeds to runtime startup; there is no `@sha256:` validation or equivalent provenance check. The controlled test failed only because this verifier image has no Docker or Podman runtime:

```text
lsp-readiness: cannot start definitely-not-a-container-runtime; install it or choose --runtime podman
exit 2
```

Require a digest-pinned image reference before invoking the container runtime.

### Low — unknown routes return HTTP 200

`/definitely-not-a-real-route` renders the designed not-found screen and title, but the response status is `200`, not `404`. This conflicts with the routing contract and masks broken links from crawlers and monitoring.

### Low — strict Rust linting fails

`cargo clippy --all-targets --all-features -- -D warnings` exits 101 for two `clippy::collapsible-if` findings in `src/lib.rs:557-568`. Build, formatting, and tests still pass.

## Mandatory first-read test

**Pass.** A cold 1280×720 visit shows:

- What: “Verify tooling before an agent edits.”
- For whom: teams onboarding contributors who need navigation, diagnostics, formatting, and tests ready.
- First action: “Try it with sample data,” with “See a finished probe in one click.”

The action is visible in the first viewport and opens `/demo` in one click. The resulting demo is visually complete, but its evidence is not truthful as described above.

## Claims manifest

`.factory/claims.json` exists and lists five tests. Before dependency installation, each exact command stopped at `vite: not found`. After the required clean-clone install (`npm ci`), every exact manifest command passed:

| Claim | Exact test | Result | Independent evidence |
| --- | --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Test passes; claim truth fails | Browser assertion: 1 passed. Actual bundled sample: 0 tests, not 42; published digest differs. |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Pass | 1 passed; live landing → demo → run → replay made only same-origin requests. |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Pass | 1 passed; independently generated packet verified; semantic tampering exited 2. |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | Pass | 1 passed; independent live service-worker update and offline `/demo` reload returned the correct H1. |
| `private-ci-price` | `npm test -- --grep @claim:private-ci-price` | Copy assertion passes; purchase flow fails | 1 passed; price/inclusions/link are present, but live checkout returns 404. |

The test commands rebuild, type-check, run all five Rust tests, and then run the selected browser test. Each selected browser assertion reported `1 passed`.

## Clean-clone quality gates

| Check | Result | Evidence |
| --- | --- | --- |
| `npm ci` | Pass | 23 packages installed; 0 vulnerabilities. |
| `npm test` | Pass | 5 Rust tests and 15 Playwright tests passed. |
| `npm run build` | Pass | `dist/site` created; release CLI copied to downloads. |
| `npm run typecheck` | Pass | Both TypeScript projects passed as part of `npm test`. |
| `cargo fmt --check` | Pass | Exit 0. |
| strict Clippy | Fail | Two `collapsible_if` warnings promoted to errors. |
| `npm audit --audit-level=high` | Pass | 0 vulnerabilities. |
| `cargo package --allow-dirty` | Pass | 51 files; 71.9 KiB compressed; package verification compiled. |

## CLI end-to-end and consumer checks

The packaged crate was extracted and installed into a new `/tmp/lsp-qa-consumer-*` root with `cargo install --path <extracted-package> --locked`.

- `--help` documents `check`, `container`, `demo`, `verify`, exit behavior, and JSON modes.
- `demo` exited 0, wrote a packet under `/tmp`, and created a mode-600 signing key.
- `verify <packet> --json` returned `{"valid":true,"algorithm":"Ed25519"}`.
- Tampering with the repository field caused signature verification to exit 2.
- A nonexistent path and an empty repository both exited 2 with actionable errors.
- The packaged sample with unavailable tools and `--skip-tests` exited 1 and wrote a valid signed non-ready packet.
- A controlled TypeScript repository with a real JSON-RPC initialize responder, formatter, and passing `npm test` exited 0. Its packet recorded definition, references, diagnostics, formatter readiness, and test readiness, then verified successfully.
- A repository with 10,001 relevant files exited 2 with `repository scan stopped at 10,000 relevant files`.
- The live downloadable Linux binary's SHA-256 exactly matched the candidate build and reported version `0.1.0`.

Docker and Podman are absent from this verifier image, so a real container-runtime launch could not be completed. Code inspection confirms `--network none`, `--read-only`, `--cap-drop=ALL`, `no-new-privileges`, read-only source mounting, and temporary workspaces.

## Live deployment identity and browser QA

The live deployment matches the candidate output byte-for-byte:

```text
index.html  9352b537641f1013608a898a68a1b4831c2dc940ab2b1d04fe2db114190cd248
main JS     b55b1fa0538628c428aac7454ba8c280e5bd1892e9fde6e8b81051a2522567c8
main CSS    b176e42d27bca99514a6e475ba5b31b3248ca1d49839899924d212f685c8628f
Linux CLI   263d85f16556df0022264387e7cd2db667bbb77951d297d6b111b33b3a2ba646
```

Desktop and 390 px mobile checks covered `/`, `/demo`, `/privacy`, `/terms`, and an unknown path.

- No console or uncaught page errors.
- One H1, `lang=en`, title, main landmark, ordered headings, and image alt text on each route.
- Playwright Axe with WCAG A/AA tags: 0 violations, including 0 serious/critical, on all five routes.
- Factory `/opt/fleet/lib/verify-url.sh`: pass; load 626 ms, no errors, one H1, main present, no missing alt text.
- Keyboard: first Tab reaches the skip link; Enter focuses main; next Tab reaches the demo action with a 3 px visible outline; Enter navigates and focuses the new H1.
- 390 px layout: no horizontal overflow; primary action remains in the first viewport.
- Injected 200% root text size: no horizontal overflow; H1 and primary action remain present.
- Reduced motion matches and collapses transitions/animations to effectively instant `0.000001s`; scroll behavior is `auto`.
- Demo traffic: no cross-origin request during landing → demo → run → replay.
- Demo storage uses only `demo:lsp-readiness-check`; reset reloads the initial demo state.
- License form: empty and invalid tokens produce clear live-region recovery messages without console errors.
- Service worker: activated, update completed, and `/demo` reloaded offline with HTTP 200 and the correct content.
- Live sample packet signature verifies, although the underlying evidence/digest is false as noted above.

## Privacy, headers, caching, and rate limit

The live HTML response includes:

- `Content-Security-Policy` with same-origin script/style/font/image defaults, only `api.sociobot.in` in `connect-src`, and `frame-ancestors 'none'` as a response header.
- `Strict-Transport-Security`, `X-Content-Type-Options: nosniff`, `Referrer-Policy`, and restrictive `Permissions-Policy`.
- HTML and service worker cache for 30 seconds; hashed JS/CSS cache for one year with `immutable`; hero image cache for one week.
- No third-party fonts, scripts, analytics, or demo requests.

The Sociobot license-verification endpoint allows 30 requests for one retained-cookie client in the observed window. Request 31 returned:

```text
HTTP/2 429
Retry-After: 3
X-RateLimit-After: 3
Too Many Requests! Wait for 3s
```

CORS allows `https://lsp-readiness-check.sociobot.in`, and verification responses use `Cache-Control: no-store`.

## Performance and metadata

Independent Lighthouse 12.8.2 mobile run against the live URL exited 0:

- Performance 100
- Accessibility 100
- Best Practices 100
- SEO 100
- FCP 0.9 s, LCP 1.8 s, total blocking time 20 ms, CLS 0

Initial resources total about 199 KB transferred: JS 6.1 KB gzip, CSS 3.7 KB gzip, font 67.3 KB, and hero WebP 120.1 KB. These are within the stated budgets.

Metadata passes: 47-character descriptive title, 86-character description, canonical URL, Open Graph/Twitter metadata, 1200×630 social image, 180×180 touch icon, robots file, and sitemap for all four real routes.

## Coverage exclusions

- No Docker or Podman runtime was available, so container execution could not be observed beyond argument handling and code inspection.
- No real purchase was attempted because the checkout endpoint fails before hosted checkout.
- Sign-in, backend persistence/concurrency, and Entra tenant checks are not applicable to this static site/local CLI.

## Required release decision

**FAIL.** Do not release until the demo is backed by truthful executable sample evidence, checkout works end to end, and the mobile accessibility failures are fixed. Enforcing digest-pinned container images is also required by the researched brief.
