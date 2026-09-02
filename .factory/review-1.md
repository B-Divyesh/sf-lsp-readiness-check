# Adversarial first-read review 1 — FAIL

- Work order: `lsp-readiness-check-review-1`
- Reviewed: 2026-09-02 UTC
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Candidate: `e9da54f96a1169c74fd53765ca90138718ad02fa`

## Verdict

**FAIL.** The real CLI, one-click demo, routing, privacy traffic, visual identity, and required tests all worked in this review. The product still makes several visitor-relevant promises that have no entry and no observable sandbox test in `.factory/claims.json`. The claims contract requires a listed test for each such promise, so this is not a zero-finding pass.

## Cold first read

Fresh Chromium contexts were opened at 1440×900 and 390×844 before scrolling. Both first screens made the job, audience, and first action clear:

| Check | Observed copy |
| --- | --- |
| What it does | “Verify tooling before an agent edits” |
| Who it is for | “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.” |
| First click | “Try it with sample data” — “See a finished probe in one click.” |

This first-read check passes. The 390 px view had no horizontal overflow. The visual treatment is product-specific: a warm survey-sheet grid, contour-map art, clipped panels, and terminal evidence, rather than a generic SaaS card layout.

## Findings

### F-1-1 — Minor: the no-account promise has no claim test

**Location/quote:** landing fact, “No account is needed for the free CLI”; README, “The bundled sample needs no account or repository setup.”

**Why this fails review:** A first-time visitor can rely on both statements when choosing whether to try the tool. Neither is covered by an entry in `.factory/claims.json`; the existing `local-operation` test proves same-origin browser traffic and source inspection, not that CLI or demo paths avoid account/authentication.

**Concrete fix:** Add a `no-account` claim and a clean browser/CLI test that runs `/demo` and `lsp-readiness demo` with no credentials, cookies, or environment key, asserts successful output, and records no authentication request. Alternatively, remove the two promises.

### F-1-2 — Minor: the landing makes an untested promise not to install or update tools

**Location/quote:** landing, “It does not install or update language servers.”

**Why this fails review:** This is a useful safety promise for repository owners, but no claim lists it. A code search is not the required observable sandbox test.

**Concrete fix:** Add an `no-tool-install` claims entry. Run `check` against the fake runtime with installer commands placed first on `PATH`; assert no installer process runs and the source mount remains unchanged. Keep the sentence only if that test passes.

### F-1-3 — Minor: the README promises no dependency installation without a test

**Location/quote:** README, Privacy and isolation, “It does not install dependencies or transmit source code.”

**Why this fails review:** The source-transmission portion is covered in substance by `local-operation`; the dependency-installation portion is a separate, actionable claim with no claim id or test.

**Concrete fix:** Split the sentence. Retain the transmission statement under `local-operation`; add a `no-dependency-install` claim using the same fake-runtime/command-trap approach as F-1-2, or delete “It does not install dependencies.”

### F-1-4 — Minor: CI non-interactivity is an unlisted behaviour claim

**Location/quote:** README, Check a repository, “The command never prompts in CI.”

**Why this fails review:** Automation users depend on this statement. No `claims.json` entry invokes `check` with non-interactive stdin and confirms it exits with its documented result rather than reading stdin.

**Concrete fix:** Add a `noninteractive-ci` claim whose test closes stdin, invokes every public command path used in CI, and asserts completion within a fixed timeout. Or remove the sentence.

### F-1-5 — Minor: signing-key permission is an unlisted security claim

**Location/quote:** README, Check a repository, “The first check creates `.lsp-readiness/signing.key` with owner-only permissions.”

**Why this fails review:** The packet-signature test verifies cryptographic validity; it does not prove filesystem mode/ownership. This matters when a team decides whether the key is safe to retain in CI.

**Concrete fix:** Add a `signing-key-permissions` claim. In a fresh temporary directory, run the public command and assert the generated key mode is `0600` on Linux. State platform-specific handling if Windows differs.

### F-1-6 — Minor: two heading labels use unexplained jargon instead of section names

**Location/quote:** landing eyebrow/headline, “Repository preflight · CLI”; landing section heading, “How the preflight works.”

**Why this fails review:** “Preflight” is an aviation metaphor and does not name a repository action for a cold visitor. It also changes the product's otherwise plain term “check.” The heading rule requires labels that make sense in a screen-reader heading list.

**Concrete fix:** Rewrite both to use the established term: “Repository check · command-line tool” and “How the repository check works.”

### F-1-7 — Minor: one landing heading is a slogan rather than a section name

**Location/quote:** landing `h2`, “Give agents evidence they can read.”

**Why this fails review:** It describes a benefit but not the section's subject. A heading list gives no name for the signed JSON output presented immediately below.

**Concrete fix:** Use “Signed capability packet” as the `h2`; keep the explanatory paragraph below it.

## Demo and sandbox

Pass. From a fresh page, one click on “Try it with sample data” opened `/demo`. Within 180 ms the first demo screen displayed a completed `northstar-api` probe with TypeScript and Rust language servers, formatters, 42 passing tests, and the Ed25519 signature. The persistent banner read “Demo — sample data, nothing is saved” and exposed Reset demo and Start for real.

Reset was exercised at 390 px: it removed the `demo:lsp-readiness-check` key and re-seeded the bundled sample on reload; no other application key appeared. The live request log for landing → demo → replay contained only `https://lsp-readiness-check.sociobot.in` requests. The independent CLI run created `/tmp/lsp-readiness-demo-7693/lsp-readiness.json`, and `verify --json` returned `{"valid":true,"algorithm":"Ed25519"}`.

## Claims manifest and clean-sandbox results

All four required commands were run after a fresh `npm ci`; all passed. The complete `npm test` also passed (11 Rust tests and 18 browser tests).

| Claim id | Exact command | Result | Confirmed observable result |
| --- | --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Pass | The shipped fixture runs 42 tests, and the displayed/published signed packet has the same source digest. |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Pass | Demo traffic is same-origin; locked-down container flags and symlink containment are exercised. |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Pass | A fresh temp packet validates as Ed25519-signed; tampering is rejected. |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | Pass | A new service-worker context reloads `/demo` offline after first visit. |

The listed claims are not failures. Findings F-1-1 through F-1-5 identify distinct claim-like statements on the live landing/README that are not in this table or `claims.json`.

## Copy audit

Counts treat a hyphenated term, code identifier, URL, and numeric token as one word. Fragments, labels, code samples, table cells, and navigation controls are listed separately after the sentences. No audited sentence is over 22 words. The two jargon/slogan flags are F-1-6 and F-1-7.

### Landing sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin. | 16 | Pass |
| See a finished probe in one click. | 7 | Pass |
| The demo reloads offline after its first visit. | 8 | Listed `offline-demo` claim |
| The CLI writes one JSON packet. | 6 | Listed `signed-packet` claim |
| It records each probe, the repository inventory digest, and an Ed25519 signature. | 12 | Listed `signed-packet` claim |
| The normal check runs in a network-disabled container made from your digest-pinned development image. | 14 | Listed `local-operation` claim |
| Detect source languages and declared test commands. | 7 | Product instruction |
| Ignore dependencies, build output, and source contents. | 7 | Product instruction |
| Start each detected language server. | 5 | Product instruction |
| Check formatter versions and run the test command. | 8 | Product instruction |
| Write a JSON capability packet. | 5 | Product instruction |
| Verify its Ed25519 signature before an agent starts work. | 9 | Listed `signed-packet` claim |
| It does not upload source code or repository file contents. | 10 | Listed `local-operation` claim |
| It does not install or update language servers. | 9 | F-1-2 |
| It does not replace your editor, test runner, or container policy. | 11 | Scope statement |

Landing labels/fragments checked: “Repository preflight · CLI” (F-1-6); “Verify tooling before an agent edits” (6, clear headline); “Try it with sample data” (6, result-naming action); “Source stays on your machine” (5, covered by `local-operation`); “No account is needed for the free CLI” (8, F-1-1); “Give agents evidence they can read” (6, F-1-7); “How the preflight works” (4, F-1-6); “Scan the repository,” “Probe each tool,” and “Sign the result” (clear action headings); “What the CLI does not do” (6, clear section name); “Download Linux binary” and “Copy command” (result-naming controls).

### README sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| Verify code navigation, diagnostics, formatting, and tests before an agent edits your repository. | 13 | Pass |
| LSP Readiness Check is a small Rust CLI for teams that onboard contributors into agent-assisted repositories. | 16 | Audience/product definition |
| It detects repository languages and starts each available language server. | 9 | Product definition |
| It checks formatters, finds tests, and writes an Ed25519-signed JSON capability packet. | 11 | Listed `signed-packet` claim |
| The bundled sample needs no account or repository setup. | 9 | F-1-1 |
| It includes tiny fixture language servers and formatters so the CLI can run the full probe. | 16 | Listed `sample-probe` claim |
| The command creates a temporary packet and prints its path. | 9 | Listed `signed-packet` claim |
| The browser version is available at the demo URL. | 8 | Direction, not a product claim |
| It uses bundled sample data, stores demo state under `demo:lsp-readiness-check`, and reloads offline after its first visit. | 13 | Listed `offline-demo` / `local-operation` claims |
| Choose the digest-pinned development image that contains your repository tools. | 10 | Required user input |
| Pass it with `--image` or set `LSP_READINESS_IMAGE`. | 7 | Required user instruction |
| The normal `check` command always creates a locked-down Docker container. | 9 | Listed `local-operation` claim |
| Choose Podman when needed. | 4 | User instruction |
| The container has no network, Linux capabilities, or writable root. | 10 | Listed `local-operation` claim |
| It receives a read-only source mount and copies that source into temporary storage. | 13 | Listed `local-operation` claim |
| The host signs the returned inventory, so the signing key is never mounted into the container. | 15 | Listed `local-operation` / `signed-packet` claims |
| Mutable image tags are rejected before a runtime starts. | 9 | Listed `local-operation` claim |
| The selected image must be Linux x86-64 with glibc, `/bin/sh`, and `cp`. | 12 | Requirement; no promise beyond documented input |
| It must contain the language tools and dependencies you want checked. | 11 | Required user input |
| `lsp-readiness container` remains as a compatibility alias for `check`. | 8 | CLI interface description |
| The first check creates `.lsp-readiness/signing.key` with owner-only permissions. | 9 | F-1-5 |
| Keep that key in your CI secret store if multiple runners must produce packets for the same policy. | 15 | User instruction |
| By default, the CLI runs the detected test command. | 9 | CLI interface description |
| Use `--skip-tests` for a fast inventory that cannot return a ready result. | 11 | CLI interface description |
| Exit codes are `0` for ready, `1` for completed checks that are not ready, and `2` for input or runtime errors. | 18 | CLI interface description |
| The command never prompts in CI. | 6 | F-1-4 |
| Test commands are detected from `package.json`, `Cargo.toml`, `pyproject.toml`, or `go.mod`. | 9 | CLI interface description |
| The inventory digest covers relevant file paths and sizes, not source contents. | 11 | Listed `signed-packet` / privacy claim |
| The CLI makes no network request and contains no telemetry. | 9 | Listed `local-operation` claim |
| Normal checks execute repository tools only inside the locked-down container. | 9 | Listed `local-operation` claim |
| The CLI skips every source-tree symlink and never mounts the signing key into the sandbox. | 13 | Listed `local-operation` claim |
| It does not install dependencies or transmit source code. | 8 | F-1-3 for the installation portion; transmission covered |
| The website makes no cross-origin request. | 6 | Listed `local-operation` claim |
| Its demo uses only bundled sample data. | 7 | Listed `local-operation` claim |
| `npm test` builds the release CLI and site, runs Rust tests, and runs browser claim and accessibility tests. | 15 | Development instruction |
| `npm run build:site` writes the static deploy to `dist/site/`. | 8 | Development instruction |
| The full build also places the Linux x86-64 CLI at `dist/site/downloads/lsp-readiness-linux-x86_64`. | 12 | Build output description |

README headings, code fences, the language-support table, repository-map bullets, links, and license reference are labels/examples rather than sentences. Controls and commands use direct verbs (`Install`, `Check`, `Verify`, `Use`).

## History regression check

Every finding in `.factory/verification.md` and `.factory/verification-2.md` was checked again in live output and code:

| Earlier finding | Current evidence | Result |
| --- | --- | --- |
| Sample falsely claimed 42 tests / stale digest | `@claim:sample-probe` executes the fixture, sees `# pass 42`, compares the packet digest, and passes. | Fixed |
| Undeployed private-CI checkout | No paid offer, checkout action, or billing endpoint is presented. | Fixed by removal |
| Mobile targets below 44 px | Live 390 px suite passes the measurement on landing and demo. | Fixed |
| Mutable image accepted | The public command rejects `ubuntu:latest`; regression passes. | Fixed |
| Unknown route returned 200 | `/no-such-route` returns 404 with the designed static not-found page. | Fixed |
| Strict Clippy failed | `cargo clippy --all-targets --all-features -- -D warnings` passed. | Fixed |
| Default check ran on host | `check` requires a digest-pinned image and the executable isolation tests pass. | Fixed |
| Directory symlink escape | Relative and absolute directory-symlink regression tests pass. | Fixed |

No earlier review or polish file exists in this checkout. The Docker/Podman real-engine limitation recorded in the earlier handoff remains environmental rather than a regression: this worker has neither runtime, while the executable fake-runtime tests verify the normal command's locked-down arguments.

## Structure, accessibility, and link checks

Pass. The live route responses were 200 for `/`, `/demo`, `/privacy`, and `/terms`; the unknown route returned HTTP 404. `robots.txt`, `sitemap.xml`, OG image, icon, sample JSON, Linux download, the external Param Factory link, and all first-page internal links resolved. Each application route has one `h1`, a `main` landmark, an appropriate route title, description, canonical URL, OG/Twitter metadata, self-hosted font and art, a favicon, and a designed 404. Navigation uses real URLs, history/back handling, focus transfer to the new `h1`, and an aria-live announcement.

`PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` passed 18/18. That included route structure, Axe WCAG A/AA serious/critical checks, keyboard skip link and focus, 390 px layout/targets, reduced motion, reset-adjacent demo checks, same-origin privacy traffic, and offline reload. The response CSP is a header and includes `frame-ancestors 'none'`; there were no browser console or page errors in the live suite.

## Missed leverage

No additional AI feature is expected: the brief calls for a local, container-isolated CLI that produces signed readiness evidence, and an AI request would make the privacy boundary less clear. Import/export is present in the useful form for this product: the signed JSON packet is downloadable from the demo and written by the CLI.

## What would make this perfect

Add the five narrow claim tests (or remove the corresponding promises), replace the two “preflight”/slogan headings with the proposed plain-language section names, then re-run the complete checklist from a clean checkout and fresh live browser contexts. A real Docker/Podman smoke run against a digest-pinned development image would provide additional operational confidence, though it is not required to resolve these findings.
