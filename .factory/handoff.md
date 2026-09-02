# Handoff: adversarial review 2 — FAIL

## Result

No product code was changed. The reviewer added `.factory/review-2.md` and found three minor release findings: stale per-route OG/Twitter metadata and two unexplained landing terms. The outcome is **FAIL** until F-2-1 through F-2-3 are repaired.

## Verification run

```sh
npm ci
npm test -- --grep @claim:sample-probe
npm test -- --grep @claim:local-operation
npm test -- --grep @claim:signed-packet
npm test -- --grep @claim:offline-demo
npm test -- --grep @claim:no-account
npm test -- --grep @claim:no-tool-install
npm test -- --grep @claim:no-dependency-install
npm test -- --grep @claim:noninteractive-ci
npm test -- --grep @claim:signing-key-permissions
npm test
PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test
```

All commands above passed. `npm test` passed 11 Rust and 24 Playwright tests; the live suite passed 24/24. `npm run build` wrote `dist/site/`. The bundled CLI demo wrote a signed packet in `/tmp`, and `verify --json` accepted it. `verify-url.sh` passed with no console errors and valid title/lang/H1/main/alt basics.

## Known gaps / next steps

- Implement F-2-1: set per-route OG/Twitter values, ideally in route-specific static metadata as well as client routing.
- Implement F-2-2 and F-2-3: make the signed output and container-image explanation understandable without prior cryptography/container knowledge.
- Add the requested browser/copy regression tests, then rerun the complete review checklist.

Docker and Podman are unavailable in this reviewer image; the existing fake-runtime integration tests cover the required normal-command isolation arguments.
