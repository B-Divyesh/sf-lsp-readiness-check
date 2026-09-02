# Adversarial first-read review 2 — FAIL

- Work order: `lsp-readiness-check-review-2`
- Reviewed: 2026-09-02 UTC
- Live URL: <https://lsp-readiness-check.sociobot.in>
- Reviewed commit: `9e99704736120ca8296bd2dc93daf7274cc981bf`

## Verdict

**FAIL.** The CLI, sample sandbox, all nine declared claim commands, local quality suite, and live browser suite work. Three minor findings remain. The page must not be accepted as PASS until its routed social metadata is truthful and the unexplained technical terms in the landing copy are made plain.

## Cold first read

Fresh Chromium contexts opened the live root at 1440×900 and 390×844 before scrolling. The first screen was unambiguous in both contexts:

| Question | Observed copy | Result |
| --- | --- | --- |
| What does it do? | “Verify tooling before an agent edits” | Clear |
| Who is it for? | “For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin.” | Clear |
| What should I click first? | “Try it with sample data” followed by “See a finished probe in one click.” | Clear |

The 390 px viewport had `scrollWidth = 390`, no console or page errors, and the action remained visible before scrolling. The visual system is recognisably product-specific: warm survey-sheet paper, contour art, clipped field panels, and a terminal recording. It is not a generic SaaS card layout.

## Findings

### F-2-1 — Minor: Demo, Privacy, and Terms retain the home page’s Open Graph and Twitter metadata

**Location/evidence:** After a direct live navigation to `/demo`, the browser correctly updates the document title, description, and canonical URL to the demo route. It still exposes `og:title` as “LSP Readiness Check — verify repository tooling”, `og:description` as “Check language servers, formatters, and tests before an agent changes your repository.”, `og:url` as `https://lsp-readiness-check.sociobot.in/`, and `twitter:title` as the home-page title. `/privacy` and `/terms` have the same mismatch. A direct HTTP GET to each route initially returns the shared home-page metadata too.

**Why this fails review:** Shared links for those real places describe the landing page instead of the page a recipient will open. This violates the per-route Open Graph/Twitter metadata requirement and makes Privacy/Terms links look unrelated.

**Concrete fix:** Extend route rendering to update `og:title`, `og:description`, `og:url`, `twitter:title`, `twitter:description`, and `twitter:image` together with `document.title`, description, and canonical. Add a browser regression that deep-links to each route and asserts all these values. Prefer route-specific static/prerendered metadata as well, so non-JavaScript link crawlers receive the right values.

### F-2-2 — Minor: The landing presents the cryptographic output in unexplained jargon

**Location/quote:** Hero terminal, “Signed: Ed25519 capability packet”; signed-output paragraph, “It records each probe, the repository inventory digest, and an Ed25519 signature.”; step copy, “Verify its Ed25519 signature before an agent starts work.”

**Why this fails review:** A first-time visitor can understand that the tool checks navigation and tests, but cannot tell what value “Ed25519” or “capability packet” adds without already knowing the cryptographic implementation. These terms are evidence details, not the user’s job, and the page does not explain them in plain language.

**Concrete fix:** Name the user outcome first and retain the implementation in parentheses where useful: use “Signed JSON readiness report” as the heading/terminal label; rewrite the paragraph to “It records each probe, a list of relevant files, and a tamper-evident signature (Ed25519).”; rewrite the step to “Verify the report’s signature before an agent starts work.” The existing `signed-packet` test already proves tamper rejection; update its visible-copy assertion to cover the new wording.

### F-2-3 — Minor: The explanation of isolation uses an undefined image-management term

**Location/quote:** Landing “How the repository check works”: “The normal check runs in a network-disabled container made from your digest-pinned development image.”

**Why this fails review:** “Digest-pinned development image” is meaningful to a container specialist, but a team onboarding a contributor gets no explanation of what they must choose or why it is safer. It is the only explanatory sentence for the product’s isolation boundary.

**Concrete fix:** Rewrite to “The normal check uses a network-disabled container made from the exact development image you choose.” In the installation section, add one short definition: “Use an image address with a SHA-256 digest so the same tools run each time.” Retain the immutable-image regression under `local-operation` and add a visible-copy assertion for this description.

## Copy audit

Counts treat a hyphenated term, code identifier, URL, and number as one word. Headings, labels, table cells, and code examples are listed separately because they are not sentences. No sentence is over 22 words. There are no marketing-adjective, inconsistent-term, metaphor-heading, or non-result-naming-button findings. F-2-2 and F-2-3 are the jargon findings.

### Landing sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin. | 16 | Pass |
| See a finished probe in one click. | 7 | Pass |
| The CLI writes one JSON packet. | 6 | Listed `signed-packet` claim |
| It records each probe, the repository inventory digest, and an Ed25519 signature. | 12 | F-2-2 |
| The normal check runs in a network-disabled container made from your digest-pinned development image. | 14 | F-2-3; listed `local-operation` claim |
| Detect source languages and declared test commands. | 7 | Product instruction |
| Ignore dependencies, build output, and source contents. | 7 | Product instruction |
| Start each detected language server. | 5 | Product instruction |
| Check formatter versions and run the test command. | 8 | Product instruction |
| Write a JSON capability packet. | 5 | Product instruction |
| Verify its Ed25519 signature before an agent starts work. | 9 | F-2-2; listed `signed-packet` claim |
| It does not upload source code or repository file contents. | 10 | Listed `local-operation` claim |
| It does not install or update language servers. | 9 | Listed `no-tool-install` claim |
| It does not replace your editor, test runner, or container policy. | 11 | Scope statement |

Landing fragments and controls checked: “Repository check · command-line tool”; “Verify tooling before an agent edits”; “Try it with sample data”; “Source stays on your machine”; “The demo reloads offline after its first visit”; “No account is needed for the free CLI”; “Signed capability packet”; “How the repository check works”; “What the CLI does not do”; “Download Linux binary”; and “Copy command”. The headline is six words, the three section headings name their sections, and all buttons/links name a destination or result. The technical labels “Ed25519”, “capability packet”, and “digest-pinned development image” are covered by F-2-2/F-2-3 rather than silently accepted.

### README sentences

| Sentence | Words | Result |
| --- | ---: | --- |
| Verify code navigation, diagnostics, formatting, and tests before an agent edits your repository. | 13 | Clear product description |
| LSP Readiness Check is a small Rust CLI for teams that onboard contributors into agent-assisted repositories. | 16 | Audience/product definition |
| It detects repository languages and starts each available language server. | 9 | Covered by `sample-probe` |
| It checks formatters, finds tests, and writes an Ed25519-signed JSON capability packet. | 11 | Listed `sample-probe`/`signed-packet` claim; see F-2-2 terminology |
| The bundled sample needs no account or repository setup. | 9 | Listed `no-account` claim |
| It includes tiny fixture language servers and formatters so the CLI can run the full probe. | 16 | Listed `sample-probe` claim |
| The command creates a temporary packet and prints its path. | 9 | Listed `signed-packet` claim |
| The browser version is available at the demo URL. | 8 | Direction |
| It uses bundled sample data, stores demo state under `demo:lsp-readiness-check`, and reloads offline after its first visit. | 13 | Listed `offline-demo`/`local-operation` claims |
| Choose the digest-pinned development image that contains your repository tools. | 10 | User instruction; see F-2-3 terminology |
| Pass it with `--image` or set `LSP_READINESS_IMAGE`. | 7 | User instruction |
| The normal `check` command always creates a locked-down Docker container. | 9 | Listed `local-operation` claim |
| Choose Podman when needed. | 4 | User instruction |
| The container has no network, Linux capabilities, or writable root. | 10 | Listed `local-operation` claim |
| It receives a read-only source mount and copies that source into temporary storage. | 13 | Listed `local-operation` claim |
| The host signs the returned inventory, so the signing key is never mounted into the container. | 15 | Listed `local-operation`/`signed-packet` claims |
| Mutable image tags are rejected before a runtime starts. | 9 | Listed `local-operation` claim |
| The selected image must be Linux x86-64 with glibc, `/bin/sh`, and `cp`. | 12 | Required input |
| It must contain the language tools and dependencies you want checked. | 11 | Required input |
| `lsp-readiness container` remains as a compatibility alias for `check`. | 8 | Interface description |
| The first check creates `.lsp-readiness/signing.key` with owner-only permissions. | 9 | Listed `signing-key-permissions` claim |
| Keep that key in your CI secret store if multiple runners must produce packets for the same policy. | 15 | User instruction |
| By default, the CLI runs the detected test command. | 9 | Interface description |
| Use `--skip-tests` for a fast inventory that cannot return a ready result. | 11 | Interface description |
| Exit codes are `0` for ready, `1` for completed checks that are not ready, and `2` for input or runtime errors. | 18 | Interface description |
| The command never prompts in CI. | 6 | Listed `noninteractive-ci` claim |
| Test commands are detected from `package.json`, `Cargo.toml`, `pyproject.toml`, or `go.mod`. | 9 | Interface description |
| The inventory digest covers relevant file paths and sizes, not source contents. | 11 | Listed `local-operation`/`signed-packet` claim |
| The CLI makes no network request and contains no telemetry. | 9 | Listed `local-operation` claim |
| Normal checks execute repository tools only inside the locked-down container. | 9 | Listed `local-operation` claim |
| The CLI skips every source-tree symlink and never mounts the signing key into the sandbox. | 13 | Listed `local-operation` claim |
| It does not install dependencies or transmit source code. | 8 | Listed `no-dependency-install`/`local-operation` claims |
| The website makes no cross-origin request. | 6 | Listed `local-operation` claim |
| Its demo uses only bundled sample data. | 7 | Listed `local-operation` claim |
| `npm test` builds the release CLI and site, runs Rust tests, and runs browser claim and accessibility tests. | 15 | Development instruction |
| `npm run build:site` writes the static deploy to `dist/site/`. | 8 | Build instruction |
| The full build also places the Linux x86-64 CLI at `dist/site/downloads/lsp-readiness-linux-x86_64`. | 12 | Build-output description |

README headings are descriptive (`Try the sandbox`, `Check a repository`, `Privacy and isolation`), commands are direct, and the support table uses the same terms as the application. The README inherits the Ed25519/digest terminology issue but adds enough surrounding technical context; the first visitor-facing instances are F-2-2/F-2-3.

## Demo, sandbox, CLI, and privacy behaviour

Pass. In a fresh browser, one click on “Try it with sample data” opened `/?demo=1`. The immediate page showed the product in use: completed `northstar-api` results (5/5, TypeScript and Rust, formatters, 42 passing tests, signed packet) plus the persistent **“Demo — sample data, nothing is saved”** banner. The banner has **Reset demo** and **Start for real**.

Reset was exercised at 390 px. It reseeded only `demo:lsp-readiness-check`; Start for real cleared that key and returned home. No other product storage key appeared. A request log for landing → demo → sample probe → replay contained only `https://lsp-readiness-check.sociobot.in` requests.

`cargo run --quiet -- demo` ran the bundled sample in a fresh temporary directory, printed `/tmp/lsp-readiness-demo-8962/lsp-readiness.json`, and `cargo run --quiet -- verify <packet> --json` returned `{"valid":true,"algorithm":"Ed25519"}`. The repository worktree was unchanged. No AI feature is expected: the brief’s value is an offline, isolated CLI proof; an AI call would make its privacy boundary worse. The useful export is present as the signed JSON download/packet.

## Claims and quality gates

`npm ci` was run first from this checkout. Each exact command from `.factory/claims.json` then completed successfully; the command loop used `set -e`, so a failed entry would have stopped the sequence. The full suite was run again afterwards.

| Claim id | Exact manifest command | Result |
| --- | --- | --- |
| `sample-probe` | `npm test -- --grep @claim:sample-probe` | Pass |
| `local-operation` | `npm test -- --grep @claim:local-operation` | Pass |
| `signed-packet` | `npm test -- --grep @claim:signed-packet` | Pass |
| `offline-demo` | `npm test -- --grep @claim:offline-demo` | Pass |
| `no-account` | `npm test -- --grep @claim:no-account` | Pass |
| `no-tool-install` | `npm test -- --grep @claim:no-tool-install` | Pass |
| `no-dependency-install` | `npm test -- --grep @claim:no-dependency-install` | Pass |
| `noninteractive-ci` | `npm test -- --grep @claim:noninteractive-ci` | Pass |
| `signing-key-permissions` | `npm test -- --grep @claim:signing-key-permissions` | Pass |

`npm test` passed after that: 11 Rust tests and 24 Playwright tests. `npm run build` produced `dist/site/`. The initial built JavaScript is 4.97 kB gzip and CSS is 3.68 kB gzip. The live suite also passed: `PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` reported 24/24.

Every visitor-reliance claim on the landing/README maps to the manifest’s sample-probe, isolation/privacy, signing, offline, account, installer, noninteractive, or permission coverage. No unlisted claim finding was found.

## History regression check

Every earlier finding was rechecked on live output and in the current code; none is merely marked fixed.

| Earlier id/finding | Current confirmation | Result |
| --- | --- | --- |
| Verification 1: false 42-test demo and stale digest | The bundled fixture test reports 42; the packet test compares the generated/published digest and verifies its signature. | Fixed |
| Verification 1: unavailable private-CI checkout | The live page exposes no paid offer, checkout, or billing endpoint. | Fixed by removal |
| Verification 1: sub-44 px mobile targets | Live 390 px suite checks all visible anchors/buttons and passes. | Fixed |
| Verification 1: mutable image accepted | Public command rejects a mutable tag before runtime; regression passes. | Fixed |
| Verification 1: unknown route returned 200 | Live unknown route is a designed static 404 with HTTP 404. | Fixed |
| Verification 1: strict Clippy failure | The current full test/build and prior recorded strict lint check pass; source contains the repaired form. | Fixed |
| Verification 2: default probe used host tools | Current normal-check integration test captures the digest-pinned locked-down container invocation. | Fixed |
| Verification 2: directory symlink escape | Rust tests cover external relative and absolute directory symlinks. | Fixed |
| F-1-1 no-account claim absent | `no-account` is in the manifest and passed. | Fixed |
| F-1-2 language-server install claim absent | `no-tool-install` is in the manifest and passed. | Fixed |
| F-1-3 dependency-install claim absent | `no-dependency-install` is in the manifest and passed. | Fixed |
| F-1-4 CI prompt claim absent | `noninteractive-ci` is in the manifest and passed. | Fixed |
| F-1-5 signing-key permissions claim absent | `signing-key-permissions` is in the manifest and passed. | Fixed |
| F-1-6 unexplained “preflight” labels | Live landing uses “Repository check” and “How the repository check works”. | Fixed |
| F-1-7 slogan heading | Live landing uses “Signed capability packet”. | Fixed, subject to F-2-2 plain-language refinement |

The Docker/Podman limitation in earlier handoffs remains environmental: no real engine is installed in this reviewer image. It is not a recurrence because executable fake-runtime tests exercise the normal command’s pinned-image and locked-down argument contract.

## Structure, accessibility, and links

Aside from F-2-1, pass. Live `/`, `/demo`, `/privacy`, and `/terms` return 200; an unknown route returns 404. `robots.txt`, sitemap, favicon, apple touch icon, OG art, sample packet, Linux download, and first-page links resolve. Browser routes have one H1, a main landmark, appropriate dynamic title/description/canonical values, designed focus movement to H1, history/back handling, a skip link, and an aria-live announcement. Header/footer are consistent and include Privacy, Terms, and Param Factory.

`/opt/fleet/lib/verify-url.sh https://lsp-readiness-check.sociobot.in <temporary-evidence-directory>` passed: 580 ms, no console errors, title, `lang=en`, one H1, main, image alt coverage, and named buttons. The live 24-test Playwright run includes serious/critical Axe WCAG A/AA checks for five routes, keyboard navigation, mobile targets, reduced motion, link crawling, offline demo reload, and cache update. Response headers include a response-header CSP with `frame-ancestors 'none'`, `nosniff`, referrer policy, permissions policy, and HSTS.

## What would make this perfect

Implement the three concrete repairs in F-2-1 through F-2-3, add the specified metadata/copy regressions, then re-run the nine manifest commands and live suite from a fresh install. The product would then have a clear first screen, a credible one-click isolated sample, verified privacy and CLI behaviour, and truthful metadata for every shared route.
