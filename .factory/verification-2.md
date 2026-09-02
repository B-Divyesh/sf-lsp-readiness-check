# Independent verification — FAIL

- Work order: `lsp-readiness-check-verify-2`
- Candidate commit: `ff5b7928b2d3e64dd505e0f953dc74fa7651b25e`
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Verified: 2026-09-02 UTC
- Result: **FAIL — do not release this candidate**

The previous demo, checkout, touch-target, image-pin, routing, and Clippy findings are fixed in this candidate. The hosted product is an exact byte match for the candidate build. Release is nevertheless blocked because the normal CLI path executes an untrusted repository's language servers and tests outside the isolated container required by the brief; its repository walk also follows directory symlinks outside the selected repository.

## Release-blocking findings

### High — the documented default probe is not isolated

The product contract requires active probes to run in isolated containers. The documented primary command is:

```sh
lsp-readiness check . --output .lsp-readiness.json
```

`src/main.rs` routes that command directly to `inspect_repository`. `src/lib.rs` then starts detected language-server executables with `Command::new(...).current_dir(root)` and runs `npm test`, `cargo test`, `python -m pytest`, or `go test ./...` directly on the host. It has no container runtime, network disablement, read-only filesystem, capability drop, or process sandbox on this path.

The optional `lsp-readiness container` command does construct a locked-down Docker/Podman invocation and rejects mutable image references, but callers must choose it explicitly. That does not make the normal, documented readiness check isolated. A repository is precisely the untrusted input this safety gate evaluates, so its LSP and test hooks must not receive host access by default.

Required fix: make `check` use the isolated-container implementation by default (with a digest-pinned image supplied through an explicit configuration mechanism), or make the unsafe host path an explicit, clearly labelled opt-in that cannot claim the brief's isolated-probe guarantee. Add an executable regression test proving that the normal command receives `--network none`, a read-only source mount, dropped capabilities, and a temporary work filesystem.

### Medium — repository discovery escapes through directory symlinks

`walk_source_files` uses `Path::is_dir()`, which follows symlinks. A fresh boundary probe created a root containing only `package.json` and a `foreign` symlink to a separate temporary directory holding `outside.ts`:

```text
$ lsp-readiness check <root> --skip-tests --json
exit 1
payload.languages = ["JavaScript / TypeScript"]
```

There was no TypeScript source under `<root>` itself. The reported language therefore came from the external `outside.ts`. In the host execution path, this can make the inventory and language server process data outside the selected repository, contrary to the privacy wording that the tool inspects the local repository and the intended containment boundary.

Required fix: use `symlink_metadata`/`file_type` to skip directory symlinks during discovery (or resolve and enforce that every resolved path remains below the selected root), and regression-test both an external directory symlink and an absolute symlink.

## Mandatory first-read test

**Pass.** A cold live desktop visit plainly answers all three questions on the first screen:

- What: “Verify tooling before an agent edits.”
- For whom: “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.”
- What to click first: “Try it with sample data,” followed by “See a finished probe in one click.”

The one-click action opened `/demo`; running its sample showed TypeScript and Rust language servers, formatters, 42 tests, and an Ed25519 signature.

## Claims manifest — all required claim commands passed

`.factory/claims.json` exists and contains four required claims. From a clean `npm ci` install, each exact command was run in sequence with `&&`; all completed successfully. Each command performs the production build, both TypeScript checks, Rust tests, and its selected browser assertion.

| Claim | Exact command | Result | Independent evidence |
| --- | --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Pass | Bundled fixture reports `# pass 42`; generated packet matches the published source digest; signature verifies. |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Pass | Fresh live `/demo` run produced no cross-origin request; only `demo:lsp-readiness-check` was stored. |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Pass | Installed CLI generated a packet; `verify --json` returned `{"valid":true,"algorithm":"Ed25519"}`. Semantic tampering exited 2 with `signature does not match the capability packet`. |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | Pass | A fresh live service-worker context reloaded `/demo` offline and rendered “Review a completed readiness probe”. |

## Clean-clone and CLI quality gates

| Check | Result | Evidence |
| --- | --- | --- |
| `npm ci` | Pass | 23 packages installed; `npm audit` reported 0 vulnerabilities. |
| `npm test` | Pass | 6 Rust tests and all 17 Playwright tests passed. |
| `npm run build` | Pass | Produced `dist/site/`; initial gzip JS is 4.88 kB and CSS is 3.69 kB. |
| `npm run typecheck` | Pass | Both TypeScript projects passed as part of `npm test`. |
| `cargo fmt --check` | Pass | Exit 0. |
| `cargo clippy --all-targets --all-features -- -D warnings` | Pass | Exit 0. |
| `cargo package --allow-dirty` | Pass | Packaged 56 files, 72.7 KiB compressed, and compiled its verification package. |

Consumer exercise: the documented `cargo install --path .` was installed into a new temporary prefix. Its public `--help` listed `check`, `container`, `demo`, and `verify`; `demo --json` produced a ready packet; `verify --json` passed; an empty repository returned exit 2 with the actionable “no source or package files found” error; and `container --image ubuntu:latest` returned exit 2 before trying a runtime with the immutable-digest explanation.

## Live deployment, browser, privacy, and headers

The live candidate is byte-identical to the local build:

```text
index.html     8d4290b3b1e88436e0276d6769ac223ba84b8365ed2ff367020b17adb643c1b8
main CSS       382b18011f90e1811a084103ce97b34fa48b7b3e8c5012c79590500a979331a2
main JS        e0dece3625f63e30a3ef8fc12236eea30c220046281e1c9dca70e202abea34bd
Linux binary   b8cc7007102ea717400fd1d72c5d7880500e91012a8acb20ca28ff7eb3ff6696
```

- `/`, `/demo`, `/privacy`, and `/terms` returned 200 with one H1, a main landmark, and no JavaScript console/page errors. Deliberately loading the real 404 returned HTTP 404 and the browser's expected network-resource 404 console message only.
- Playwright Axe WCAG A/AA scans found zero serious or critical violations on all five routes, including the 404.
- Desktop and 390 px mobile were visually inspected. Mobile had no horizontal overflow; all visible links/buttons met 44 px in the checked product tests. Keyboard Tab first focused the clearly visible skip link; Enter focused main; the following Tab reached the sample-demo action. `prefers-reduced-motion: reduce` reduced the route animation to `0.000001s`.
- The full live demo flow made zero cross-origin requests and emitted no console or page error. Its only localStorage key was `demo:lsp-readiness-check`; reset returned it to the initial sample state.
- The live document has `Strict-Transport-Security`, `X-Content-Type-Options: nosniff`, `Referrer-Policy`, restrictive `Permissions-Policy`, and a response-header CSP with `default-src 'self'`, `connect-src 'self'`, and `frame-ancestors 'none'`. HTML/service worker are 30-second revalidated; hashed JS is one-year immutable.
- There are no product server-side endpoints, unlock calls, authentication, or billing UI in this candidate; API rate-limit and Entra checks are not applicable.

## Scope exclusions

Docker and Podman are unavailable in this verifier image, so a successful runtime launch could not be observed. Static inspection and the existing regression test confirm that the *optional* container command supplies `--network none`, `--read-only`, `--cap-drop=ALL`, `no-new-privileges`, a read-only source mount, and tmpfs work paths. That is not a substitute for isolating the normal command.

## Release decision

**FAIL.** The product should not be released until the default readiness probe is isolated and symlink traversal is contained. All other tested quality, demo, privacy-traffic, accessibility, packaging, and live-deployment checks passed.
