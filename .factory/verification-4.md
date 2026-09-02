# Independent verification — PASS

- Work order: `lsp-readiness-check-verify-4`
- Candidate commit: `b7066e81ee076109362c5a351cc681e446035eb8`
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Verified: 2026-09-02 UTC
- Decision: **PASS — release candidate accepted**

## Mandatory cold first read

**Pass.** A fresh 1280×720 visit to the live URL displayed the following on its first screen, in plain words:

- What it does: “Verify tooling before an agent edits”.
- Who it is for: teams onboarding contributors who need navigation, diagnostics, formatting, and tests ready before changes.
- What to do first: the visible “Try it with sample data” action, followed by “See a finished probe in one click.”

The action enters the bundled `northstar-api` demo in one click. `/demo` also has the persistent “Demo — sample data, nothing is saved” banner, Reset demo, and Start for real controls.

## Required claim tests — run first

After `npm ci` in this clean checkout, every exact command in `.factory/claims.json` exited 0. Each command rebuilt the production site, type-checked both TypeScript projects, ran the Rust suite, and exercised the named browser/CLI assertion.

| Claim | Command | Result |
| --- | --- | --- |
| Sample probe | `npm test -- --grep @claim:sample-probe` | Pass — bundled probe reports both LSPs, formatters, 42 tests, and an Ed25519 packet. |
| Local operation | `npm test -- --grep @claim:local-operation` | Pass — demo traffic is same-origin; normal checks use the locked-down runtime contract. |
| Signed packet | `npm test -- --grep @claim:signed-packet` | Pass — generated packet verifies. |
| Offline demo | `npm test -- --grep @claim:offline-demo` | Pass — a fresh service-worker context reloads the demo offline. |
| No account | `npm test -- --grep @claim:no-account` | Pass — website and CLI demo work without credentials or auth routes. |
| No tool install | `npm test -- --grep @claim:no-tool-install` | Pass — command traps did not run and source stayed unchanged. |
| No dependency install | `npm test -- --grep @claim:no-dependency-install` | Pass — dependency installer traps did not run and source stayed unchanged. |
| Noninteractive CI | `npm test -- --grep @claim:noninteractive-ci` | Pass — public commands complete with stdin closed. |
| Signing-key permissions | `npm test -- --grep @claim:signing-key-permissions` | Pass — first normal check creates a mode-0600 key on Linux. |

## Clean checkout, build, and CLI package

| Check | Result | Evidence |
| --- | --- | --- |
| `npm ci` | Pass | Installed 23 packages; audit reported no vulnerabilities. |
| `npm test` | Pass | 11 Rust tests and 24 Playwright tests passed. |
| `npm run build` | Pass | Produced `dist/site/`. |
| `cargo fmt --check` | Pass | Exit 0. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Pass | Exit 0. |
| `cargo package --allow-dirty` | Pass | Packaged 58 files (79.0 KiB compressed) and compiled the verification package. |

A clean consumer install from `target/package/lsp-readiness-check-0.1.1` succeeded with `cargo install --path … --root <temporary-prefix> --locked`. Its public help documented `check`, `container`, `demo`, and `verify`. `demo` created a signed packet, `verify --json` returned `{"valid":true,"algorithm":"Ed25519"}`, and invalid input recovered correctly: an empty repository failed with the required `--image` explanation and a mutable `ubuntu:latest` image failed before runtime launch with the immutable-digest message.

## Live deployment QA

The live deployment is byte-identical to the candidate build:

```text
index.html       1764a24dcb28c04bb7e73ec08913e6b0f72540bb02c4aef3497c1d2325422bc8
main JavaScript  43003664245d0c8f9bbcc0002d9d84f758cc8b868fe189f13342be61a6b13815
main CSS         d9ab7665da6abd91151642632867314f15636b53805bc7b2f738fdeba2ff5639
Linux CLI        ce2470c3c4f1d51c08031f8c228736b08a491aa840cf18615dd786cded503c9a
```

- `/`, `/demo`, `/privacy`, and `/terms` returned 200; the designed unknown route returned HTTP 404.
- Live Playwright suite: **24/24 passed**. It covers routes, demo isolation/reset, service-worker cache update and offline reload, internal links, keyboard navigation, mobile target sizes, and Axe WCAG A/AA checks.
- Independent Axe checks found zero serious or critical findings on `/`, `/demo`, `/privacy`, `/terms`, and `/does-not-exist`.
- `/opt/fleet/lib/verify-url.sh` passed: 627 ms cold load, no console errors, title, `lang=en`, one H1, main landmark, no missing image alt text, and no unlabeled buttons.
- At 390 px the page had `scrollWidth == clientWidth == 390`; the primary demo action remained visible. Focus was visibly designed (observed 3 px solid outline), and reduced-motion animation/transition durations were `0.000001s`.
- A fresh demo run/replay made only same-origin requests, emitted no console or page errors, and stored only `demo:lsp-readiness-check`. Its displayed terminal result includes both language servers, formatters, 42 passed tests, and Ed25519 signing.
- With service workers allowed, the live `/demo` activated cache `lsp-readiness-v3` and reloaded offline with HTTP 200 and the expected demo H1.
- Headers are appropriate: response-header CSP with `connect-src 'self'` and `frame-ancestors 'none'`, HSTS, `nosniff`, strict referrer policy, and restrictive permissions policy. HTML and service worker revalidate at 30 seconds; hashed CSS/JS are one-year immutable. Initial gzip JS is 4.97 kB and CSS is 3.68 kB; the 66 kB self-hosted font and 118 kB hero image also meet the stated static budgets.

## Scope notes

There are no product server-side endpoints, authentication, billing/unlock calls, or analytics requests in this candidate. Rate-limit/429 and Entra checks are therefore not applicable.

Docker and Podman are not installed in this verifier environment, so a successful real container-engine launch could not be observed. This is not a release defect: the normal-command regression tests passed with an executable fake runtime and assert pinned-image validation, `--network none`, read-only source mount, `--read-only`, dropped capabilities, `no-new-privileges`, temporary work filesystems, host-side signing, no installers, and source immutability. A release-environment smoke test using the team’s real digest-pinned image remains a sensible follow-up.

## Defects

No release-blocking, high, medium, or low defects found.
