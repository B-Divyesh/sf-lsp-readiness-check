# Handoff: polish 1 complete

## Result

Repair commit `d2715ad` resolves F-1-1 through F-1-7 and is pushed to `main`. The static site was deployed as Static Web Apps deployment `a6d8b840-f913-40fd-9be9-a69122456956` at <https://lsp-readiness-check.sociobot.in>.

The landing now opens the isolated sample directly at `/?demo=1`, with a persistent no-save banner, reset, and exit-to-real controls. The five review promises are each registered in `.factory/claims.json` and have one observable `@claim:` test. The two review headings now use direct, plain-language names.

## How to run

```sh
npm ci
npm test
npm run build
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
```

The website build is `dist/site/`. Run `lsp-readiness demo` for the bundled CLI sample, or open `/?demo=1` for the browser sandbox.

## Exact verification evidence

From a clean clone at `/tmp/lsp-readiness-clean-XWy0kn` after `npm ci`, all of these passed individually:

```sh
npm test -- --grep @claim:sample-probe
npm test -- --grep @claim:local-operation
npm test -- --grep @claim:signed-packet
npm test -- --grep @claim:offline-demo
npm test -- --grep @claim:no-account
npm test -- --grep @claim:no-tool-install
npm test -- --grep @claim:no-dependency-install
npm test -- --grep @claim:noninteractive-ci
npm test -- --grep @claim:signing-key-permissions
```

The same clean clone then passed `npm test` (11 Rust tests and 24 Playwright tests), `npm run build`, `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo package --allow-dirty`.

After deployment, `/opt/fleet/lib/verify-url.sh https://lsp-readiness-check.sociobot.in /tmp/lsp-readiness-polish-1-live-UJ5l8q` passed with a 673 ms cold load, no console errors, a title, `lang=en`, one H1, a main landmark, and no missing image alt text. Screenshots and JSON are at:

- `/tmp/lsp-readiness-polish-1-live-UJ5l8q/screenshot-desktop.png`
- `/tmp/lsp-readiness-polish-1-live-UJ5l8q/screenshot-mobile.png`
- `/tmp/lsp-readiness-polish-1-live-UJ5l8q/verify.json`

`PLAYWRIGHT_BASE_URL=https://lsp-readiness-check.sociobot.in npx playwright test` passed 24/24. This includes live routing, metadata, focus/skip link, mobile targets and overflow, reduced motion, resettable demo isolation, offline reload, same-origin traffic, and Axe WCAG A/AA serious/critical checks. Live HTTP checks confirmed 200 for the real routes/assets and 404 for `/does-not-exist`; response headers include CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, strict referrer policy, and permissions policy.

## Known gaps

No unresolved product or review finding remains. Docker and Podman are not installed in this worker, so a successful real engine launch was not possible here; the public-command fake-runtime integration tests verify the required pinned-image, no-network, read-only source, capability-drop, tmpfs, host-signing, and command-trap behavior.

## Publishing

Do not publish from this worker. The package is ready for the factory to publish with `cargo package` after its normal registry process.
