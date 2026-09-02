# Handoff: LSP Readiness Check v0.1.0

## Shipped

- A Rust CLI with `check`, `container`, `demo`, and `verify` commands.
- Repository detection for JavaScript/TypeScript, Rust, Python, Go, and Svelte.
- Real JSON-RPC `initialize` handshakes with five-second timeouts.
- Required checks for definition, reference, and diagnostic support.
- Formatter version checks and repository test execution, with an explicit non-ready skip mode.
- Ed25519 signing with owner-only local keys and tamper verification.
- A locked-down Docker or Podman path with no network, dropped capabilities, a read-only root, and a temporary source copy.
- A bundled `northstar-api` demo and a verifiable sample packet.
- A responsive Vite site with `/`, `/demo`, `/privacy`, `/terms`, and styled 404 handling.
- A one-click browser demo with isolated `demo:` storage, reset, and exit controls.
- The Sociobot checkout link, returned-license storage, daily verification cache, and license restore form.
- An offline service worker, social metadata, sitemap, robots file, security headers, and immutable asset caching.
- Original topographic hero and social art generated through the factory image deployment.

## Run and verify

```sh
npm install
npm test
npm run build
cargo run -- demo
cargo package --allow-dirty
```

The exact static build command is `npm run build:site`. Its deploy root is `dist/site`, with `index.html` at that root. The full `npm run build` also compiles and copies the Linux x86-64 binary.

Final local results on 2026-09-02:

- `npm test`: 5 Rust tests and 15 Playwright tests passed.
- Axe: no serious or critical findings on home, demo, privacy, terms, or 404 routes.
- Factory `verify-url.sh`: title, language, one H1, main landmark, alt text, and console checks passed.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100, SEO 100.
- Lighthouse lab metrics: LCP 2.1 s, CLS 0, total blocking time 20 ms.
- Initial application JavaScript: 6.01 KB gzip.
- Initial CSS: 3.64 KB gzip.
- Self-hosted font: 66 KB. Hero WebP: 118 KB.
- `npm audit --audit-level=high`: no vulnerabilities.

## Known gaps

- The container command was compiled and its help path was checked, but no Docker or Podman runtime exists in this worker image. Run one smoke probe with the production team’s pinned development image.
- The factory must register the paid product and connect its private CI service after repository review. The site already uses the required slug-based checkout and verification endpoints.
- Registry publishing is intentionally left to the factory. `cargo package --allow-dirty` prepares the crate locally.

## Next steps

1. Run `lsp-readiness container . --image <pinned-development-image>` in pilot CI.
2. Register `lsp-readiness-check` with the Sociobot billing service.
3. Publish the reviewed crate and attach platform binaries to a release.
