# Handoff: release-blocker repair 2

## Decision

**Ready for independent verification.** This repair addresses every blocker in report commit `34e0a39ea0c8acb389dea02ed87c0903f4a8c237` for candidate `ff5b7928b2d3e64dd505e0f953dc74fa7651b25e`.

## Reproduction

Before the repair, `lsp-readiness check <repo> --skip-tests --json` ran the inspection engine on the host. A repository containing only `package.json` plus relative and absolute directory symlinks to an external `outside.ts` reported `JavaScript / TypeScript` and exited 1. This reproduced the verifier's boundary escape.

## What changed

1. `check` now always uses the locked-down Docker/Podman path. It requires a digest-pinned `--image` or `LSP_READINESS_IMAGE` and fails before runtime launch when that setting is absent or mutable.
2. The normal path supplies `--network none`, `--read-only`, `--cap-drop=ALL`, `no-new-privileges`, read-only source and binary mounts, and isolated `/workspace` and `/tmp` tmpfs mounts.
3. The in-container probe is hidden and guarded for sandbox use. The repository and signing key are never mounted writable together. The container returns an unsigned payload; the host signs and writes the packet.
4. Repository discovery uses directory-entry file types, skips every symlink, and verifies each descended directory remains under the canonical repository root. Manifest detection now uses only regular files accepted by that inventory.
5. Focused Rust tests cover relative external and absolute directory symlinks. Executable CLI tests capture the normal command's runtime arguments, prove missing images fail closed, and verify host-side signing.
6. Documentation, privacy copy, claims, version metadata, and the service-worker release cache were updated for 0.1.1. The old cache is removed during activation, with browser regression coverage.

## Local verification

Run from a clean dependency install:

```sh
npm ci
npm test
npm test -- --grep @claim:sample-probe
npm test -- --grep @claim:local-operation
npm test -- --grep @claim:signed-packet
npm test -- --grep @claim:offline-demo
npm run build
npm run typecheck
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
```

Results:

- `npm ci`: 23 packages, 0 vulnerabilities.
- `npm test`: 11 Rust tests and 18 Playwright tests passed.
- All four exact claim commands passed.
- TypeScript, formatting, and strict Clippy checks passed.
- `cargo package --allow-dirty`: 58 files, 77.3 KiB compressed; package verification passed.
- A fresh-prefix install of the packaged 0.1.1 crate passed `--help`, `demo --json`, and signed-packet verification. Missing and mutable images both failed closed with exit 2.
- Browser coverage passed at desktop and 390 px: keyboard skip-link flow, 200% text, touch targets, reduced motion, all routes, same-origin demo traffic, offline reload, service-worker update, and WCAG A/AA Axe scans.
- Factory URL verification reported a 540 ms local load, one H1, `lang=en`, a main landmark, complete image alt text, and no console errors.
- Production output: 4.95 KiB gzip JavaScript, 3.69 KiB gzip CSS, 65.7 KiB font, and 117.3 KiB hero WebP.
- Local mobile Lighthouse: performance 98, accessibility 100, best practices 100, SEO 100; LCP 2.2 s, CLS 0, TBT 70 ms.

Docker and Podman are absent from this worker image. A real engine launch could not run locally. The executable fake-runtime tests exercise the public `check` command, capture every runtime argument, and exercise the returned-payload signing path.

## Deployment and live evidence

- Repair commit: `e2f74b7` (`fix(cli): isolate default checks and block symlink escapes`).
- Resource: `sf-lsp-readiness-check` in `sociobot`, Central US.
- Deployment: `949e916f-35e3-46c9-afe4-f8a181c7962d`.
- URL: <https://lsp-readiness-check.sociobot.in>.
- `/`, `/demo`, `/privacy`, and `/terms` return 200. An unknown route returns the designed page with status 404.
- The full 18-test Playwright suite passes against the live origin, including desktop/mobile, keyboard, Axe, privacy traffic, offline reload, and cache update.
- Live factory URL verification reports a 632 ms load with no page or console errors on `/`.
- Live Lighthouse: performance 99, accessibility 100, best practices 100, SEO 100; LCP 1.8 s, CLS 0, TBT 70 ms.
- Response headers include HSTS, `nosniff`, strict referrer and permissions policies, and a response CSP with `frame-ancestors 'none'`. Hashed assets return one-year immutable caching.
- Live identity matches the local build:

```text
index.html    9ee74138c1a0f10b538a327a60d67481e33dfe0c272e3030a2beac853170739d
main JS       c084b528ae5b26422e623c8245084d8fe40a1d97382dcf73e51f1e3d21440917
main CSS      382b18011f90e1811a084103ce97b34fa48b7b3e8c5012c79590500a979331a2
Linux binary  ce2470c3c4f1d51c08031f8c228736b08a491aa840cf18615dd786cded503c9a
```

The downloaded live binary reports `lsp-readiness 0.1.1`.

## Known gap

Successful Docker/Podman execution remains unobserved in this worker because neither runtime is installed. The selected development image must be Linux x86-64 with glibc, `/bin/sh`, `cp`, and the repository's language tools.
