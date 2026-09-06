# LSP Readiness Check

Verify code navigation, diagnostics, formatting, and tests before an agent edits your repository.

LSP Readiness Check is a small Rust CLI for teams that onboard contributors into agent-assisted repositories. It detects repository languages and starts each available language server. It checks formatters, finds tests, and writes a signed JSON readiness report. The report signature makes tampering detectable (Ed25519).

The M2 service foundation accepts signed, source-free report payloads for private repositories. It stores each team in a separate SQLite tenant on `/data`. Hosted sign-in, GitHub connection, and subscriptions remain unavailable until their operator registrations pass real product QA.

Live site: <https://lsp-readiness-check.sociobot.in>

## Try the sandbox

The bundled sample needs no account or repository setup. It includes tiny fixture language servers and formatters so the CLI can run the full probe:

```sh
cargo run -- demo
```

The command creates a temporary readiness report and prints its path. The browser version is available at <https://lsp-readiness-check.sociobot.in/demo>. It uses bundled sample data, stores demo state under `demo:lsp-readiness-check`, and reloads offline after its first visit.

## Install

Build from a clean clone with stable Rust:

```sh
cargo install --path .
lsp-readiness --help
```

The factory publishes packages after review. This repository does not publish itself.

## Check a repository

Choose the exact development image that contains your repository tools. Use an image address with a SHA-256 digest so the same tools run each time. Pass it with `--image` or set `LSP_READINESS_IMAGE`:

```sh
cd your-repository
export LSP_READINESS_IMAGE='your-registry/your-dev-image@sha256:YOUR_64_HEX_DIGEST'
lsp-readiness check . --output .lsp-readiness.json
```

The normal `check` command always creates a locked-down Docker container. Choose Podman when needed:

```sh
lsp-readiness check . --runtime podman
```

The container has no network, Linux capabilities, or writable root. It receives a read-only source mount and copies that source into temporary storage. The host signs the returned inventory, so the signing key is never mounted into the container. Mutable image tags are rejected before a runtime starts.

The selected image must be Linux x86-64 with `/bin/sh` and `cp`. Its glibc version must be compatible with the installed CLI binary. It must contain the language tools and dependencies you want checked.

`lsp-readiness container` remains as a compatibility alias for `check`.

The first check creates `.lsp-readiness/signing.key` with owner-only permissions. Keep that key in your CI secret store if multiple runners must produce readiness reports for the same policy.

By default, the CLI runs the detected test command. Use `--skip-tests` for a fast inventory that cannot return a ready result:

```sh
lsp-readiness check .
lsp-readiness verify .lsp-readiness.json
```

Use `--json` for scripts:

```sh
lsp-readiness check . --json > readiness-output.json
lsp-readiness verify .lsp-readiness.json --json
```

Exit codes are `0` for ready, `1` for completed checks that are not ready, and `2` for input or runtime errors. The command never prompts in CI.

## What it detects

| Repository source | Language server | Formatter |
| --- | --- | --- |
| JavaScript / TypeScript | `typescript-language-server --stdio` | `prettier` |
| Rust | `rust-analyzer` | `rustfmt` |
| Python | `pyright-langserver --stdio` | `ruff` |
| Go | `gopls serve` | `gofmt` |
| Svelte | `svelteserver --stdio` | `prettier` |

Test commands are detected from `package.json`, `Cargo.toml`, `pyproject.toml`, or `go.mod`. The inventory digest covers relevant file paths and sizes, not source contents.

## Privacy and isolation

The CLI makes no network request and contains no telemetry. Normal checks execute repository tools only inside the locked-down container. The CLI skips every source-tree symlink and never mounts the signing key into the sandbox. It does not install dependencies or transmit source code.

The website demo makes no cross-origin request. It uses only bundled sample data.

The optional private API accepts only a bounded readiness-report schema plus repository, pull request, and run identifiers. It rejects extra source fields and secret-like evidence. Stored rows include team membership, repository names, policies, report metadata, and subscription status. Account owners can export or delete their team's stored data.

See [Privacy](https://lsp-readiness-check.sociobot.in/privacy) and [Terms](https://lsp-readiness-check.sociobot.in/terms).

## Private CI foundation

The private service is a Rust API in `server/`. Its implemented local contract includes:

- CIAM JWT validation against the configured issuer, audience, and JWKS.
- A one-time GitHub App installation state and server-side installation-token exchange.
- Organization-scoped repositories, policies, readiness runs, and subscription records.
- Signed-report verification, a 64 KB body limit, strict JSON fields, and evidence checks.
- SQLite migrations, backup/restore commands, export/delete, request IDs, health, aggregate metrics, and `429` responses with `Retry-After`.

Start the service locally with release-disabled test identity support:

```sh
DATABASE_PATH=target/local-api.db \
PUBLIC_ORIGIN=http://127.0.0.1:4173 \
API_ORIGIN=http://127.0.0.1:8787 \
PORT=8787 \
LSP_READINESS_TEST_AUTH=1 \
cargo run -p lsp-readiness-api -- serve
```

Release builds ignore `LSP_READINESS_TEST_AUTH`. Production account routes stay closed unless all CIAM settings are present. GitHub connection stays closed unless all GitHub App settings are present.

Back up or restore SQLite while the service is stopped:

```sh
DATABASE_PATH=/data/lsp-readiness.db lsp-readiness-api backup /data/backups/lsp-readiness.db
DATABASE_PATH=/data/lsp-readiness.db lsp-readiness-api restore /data/backups/lsp-readiness.db
```

The researched private plan is $49 per repository each month for private CI checks, policy templates, and readiness history. It is not available for purchase yet. Sociobot recurring subscription registration and test-mode entitlement QA are operator dependencies; no one-time license flow is substituted.

## Operator dependencies for hosted M2

- Register the Sociobot Entra CIAM application and provide its issuer, audience, client ID, authorize URL, token URL, JWKS URL, and delegated scope.
- Register the product GitHub App, callback URL, app ID, slug, and server-held signing key. Complete a real authorized installation.
- Register the recurring Sociobot subscription at $49 per repository each month. Provide and test the subscription entitlement contract.
- Deploy the API as `sf-lsp-readiness-check-api` with SQLite at `/data/lsp-readiness.db`, one replica, and the `/healthz` probe.

Do not claim sign-in, GitHub installation, checkout, or entitlement works until those hosted paths pass product QA.

## Develop and verify

Requirements: Rust, Node.js 20 or later, and the preinstalled Playwright Chromium browser.

```sh
npm ci
npm test
npm run build
```

`npm test` builds the CLI, API, and site. It runs Rust tests plus browser, API, claim, and accessibility tests. `npm run build:site` writes the static deploy to `dist/site/`. The full build also places the Linux x86-64 CLI at `dist/site/downloads/lsp-readiness-linux-x86_64`.

## Deploy

The factory deploys the built static site from `dist/site/`. Build it with `npm run build`; deployment credentials and DNS remain factory-managed.

The API image builds from `server/Dockerfile`. Its durable deployment contract is in `.factory/deploy-m2.json`. Do not run SQLite with multiple replicas or without the `/data` mount.

Package readiness can be checked without publishing:

```sh
cargo package --allow-dirty
```

## Repository map

- `src/`: CLI library and command.
- `examples/northstar-api/`: bundled demo source.
- `site/`: Vite site and browser demo.
- `server/`: Rust/Axum API and SQLite migrations.
- `tests/`: claim and accessibility tests.
- `.factory/`: brief, design, demo, claims, copy audit, and handoff.

## License

MIT. See [LICENSE](LICENSE).
