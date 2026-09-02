# Independent verification — PASS

**Candidate:** `a97c696dd6b19b24c1f1904457937a96cd45f51b`  
**URL:** <https://lsp-readiness-check.sociobot.in>  
**Verified:** 2026-09-02

## Decision

**PASS.** The deployed site is the candidate build, the CLI package works from a clean consumer prefix, all required claim tests and the complete local and live browser suites pass, and no release-blocking defect was found.

## Required claims (run first, from this clean checkout)

All passed (exit status 0):

| Claim | Exact command | Evidence |
| --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Bundled northstar-api probe reports TypeScript and Rust LSPs, formatters, 42 tests, and a signed packet. |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Browser demo traffic is same-origin; CLI tests assert locked-down runtime flags and no network client. |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Demo writes an Ed25519 packet which `verify --json` accepts. |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | A new context reloads `/demo` offline after its first visit. |

## Local and consumer verification

- `npm ci`: completed, 23 packages audited, 0 vulnerabilities.
- `npm test`: passed — 11 Rust tests and 18 Playwright tests.
- `npm run build`, `npm run typecheck`, `cargo fmt --check`, and `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo package --allow-dirty`: packaged and verified 58 files (77.3 KiB compressed).
- Installed with `cargo install --path . --root <fresh-temp-prefix>`; `--help`, `demo`, `verify <packet> --json`, and tamper rejection worked. A tampered packet exited 2 with `signature does not match the capability packet`.
- Boundary/error paths: missing image and mutable `alpine:latest` image both failed closed with exit 2 before runtime use.

## Live QA

Cold first read of `/` clearly says what it does, for whom, and what to do first:

- “Verify tooling before an agent edits.”
- “For teams onboarding contributors …”
- “Try it with sample data” with “See a finished probe in one click.”

The first screen includes the required one-click demo and three plain facts. The demo loads bundled northstar-api data, displays 5/5 ready checks and the signed result, uses only `demo:lsp-readiness-check` storage, and displays “Demo — sample data, nothing is saved” with Reset demo and Start for real.

- `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test`: **18/18 passed**. This independently covers routes, one H1/main, internal links, keyboard/skip link, 390px mobile and target size, reduced motion, offline reload, service-worker cache update, same-origin traffic, and Axe WCAG A/AA serious/critical findings.
- Fresh browser request log during landing → demo → sample probe contained only `https://lsp-readiness-check.sociobot.in` requests. No page or console errors occurred.
- Keyboard focus on the primary demo action was a visible `rgb(192, 72, 34) solid 3px` outline. At 390px, `scrollWidth` was 390 and the reduced-motion sample completed without animation delay.
- `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns the designed 404 with status 404. `robots.txt`, `sitemap.xml`, OG art, hero art, and the external Param Factory footer link return 200.
- Headers include HSTS, `nosniff`, strict referrer policy, permissions policy, and a response CSP with `frame-ancestors 'none'`; document cache is 30 seconds and hashed JS is `max-age=31536000, immutable`.
- Assets meet static budgets: JS 4,981 bytes gzip, CSS 3,690 bytes gzip, font 67,304 bytes, hero WebP 120,118 bytes. An attempted independent Lighthouse run could not start in this container because Chromium crashed under Lighthouse; browser/Axe and bundle checks above completed successfully.

## Deployment identity

Local candidate artifacts and live artifacts have identical SHA-256 values:

```text
index.html    9ee74138c1a0f10b538a327a60d67481e33dfe0c272e3030a2beac853170739d
main JS       c084b528ae5b26422e623c8245084d8fe40a1d97382dcf73e51f1e3d21440917
main CSS      382b18011f90e1811a084103ce97b34fa48b7b3e8c5012c79590500a979331a2
Linux binary  ce2470c3c4f1d51c08031f8c228736b08a491aa840cf18615dd786cded503c9a
```

The downloaded live binary reports `lsp-readiness 0.1.1`.

## Limitation / follow-up

Neither Docker nor Podman is installed in this verifier image, so a successful real container-engine launch was not directly observed. This is not a failure of the candidate: the normal-command integration tests passed using an executable fake runtime and assert `--network none`, read-only source, dropped capabilities, no-new-privileges, tmpfs workspace, pinned image validation, and host-side signing. A release-environment smoke test against the team's actual digest-pinned development image remains a sensible follow-up.

No server-side product API or factory unlock endpoint is present, so rate-limit/429 testing and sign-in checks are not applicable.
