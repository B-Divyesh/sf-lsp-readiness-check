# Polish 1 — adversarial review repairs

- Work order: `lsp-readiness-check-polish-1`
- Repair commit: `d2715ad`
- Deployed URL: <https://lsp-readiness-check.sociobot.in>
- Deployment: Static Web Apps deployment `a6d8b840-f913-40fd-9be9-a69122456956`
- Live evidence: `/tmp/lsp-readiness-polish-1-live-UJ5l8q/screenshot-desktop.png`, `/tmp/lsp-readiness-polish-1-live-UJ5l8q/screenshot-mobile.png`, and `/tmp/lsp-readiness-polish-1-live-UJ5l8q/verify.json`

## Review finding map

| Finding | Repair | Evidence |
| --- | --- | --- |
| F-1-1 | Registered the no-account promise as `no-account`. The test starts the `?demo=1` sandbox in a fresh context, records requests, and runs `lsp-readiness demo` with only `PATH` in its environment. | Clean clone: `npm test -- --grep @claim:no-account` passed. Live 24/24 browser suite passed at the deployed URL. |
| F-1-2 | Registered `no-tool-install`. Its isolated normal-check test places language-server and installer traps first on `PATH`, then proves no trap ran and the source file is unchanged. | Clean clone: `npm test -- --grep @claim:no-tool-install` passed. |
| F-1-3 | Registered `no-dependency-install`. Its independent trap test covers `npm`, package managers, language package managers, and build installers, and verifies the source file stays unchanged. | Clean clone: `npm test -- --grep @claim:no-dependency-install` passed. |
| F-1-4 | Registered `noninteractive-ci`. It invokes `check`, `container`, `demo`, and `verify` with stdin closed and a three-second completion bound. | Clean clone: `npm test -- --grep @claim:noninteractive-ci` passed. |
| F-1-5 | Registered `signing-key-permissions`. A fresh public `check` against a fake runtime now asserts the created key has Linux mode `0600`. | Clean clone: `npm test -- --grep @claim:signing-key-permissions` passed. |
| F-1-6 | Replaced “Repository preflight · CLI” with “Repository check · command-line tool”, changed “How the preflight works” to “How the repository check works”, and removed the remaining display-only “preflight” labels. | Landing route test asserts both direct labels; live browser suite passed. |
| F-1-7 | Replaced the slogan heading “Give agents evidence they can read” with the direct section name “Signed capability packet”. | Landing route test asserts the heading; live browser suite passed. |

## Additional acceptance work

- The primary sample action now opens the isolated direct URL `/?demo=1`. It displays the persistent **Demo — sample data, nothing is saved** banner, **Reset demo**, and **Start for real**. A browser test proves the separate `demo:lsp-readiness-check` namespace is reseeded on reset and cleared on exit.
- Updated the copy audit and added the verb-first 48-character catalog description: “Verify repository tooling before an agent edits.”
- The deployed site was cold-checked after upload. `/`, `/?demo=1`, `/demo`, `/privacy`, `/terms`, `robots.txt`, and `sitemap.xml` returned 200; `/does-not-exist` returned 404. CSP, HSTS, `nosniff`, referrer, and permissions headers are present.
- `/opt/fleet/lib/verify-url.sh` reported title, `lang=en`, one H1, a main landmark, image alt coverage, and no console errors. The live Playwright Axe WCAG A/AA scan passed as part of the 24-test suite.
- The earlier verification findings remain covered: truthful sample probe/digest, no billing UI, 44 px mobile controls, immutable image validation, real 404, strict Clippy, default container isolation, and symlink containment.
