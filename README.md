# LSP Readiness Check

Verify code navigation, diagnostics, formatting, and tests before an agent edits your repository.

LSP Readiness Check is a small Rust CLI for teams that onboard contributors into agent-assisted repositories. It detects repository languages and starts each available language server. It checks formatters, finds tests, and writes a signed JSON readiness report. The report signature makes tampering detectable (Ed25519).

Live site: <https://lsp-readiness-check.sociobot.in>

## Try the sandbox

The bundled sample needs no account or repository setup. It includes tiny fixture language servers and formatters so the CLI can run the full probe:

```sh
cargo run -- demo
```

The command creates a temporary packet and prints its path. The browser version is available at <https://lsp-readiness-check.sociobot.in/demo>. It uses bundled sample data, stores demo state under `demo:lsp-readiness-check`, and reloads offline after its first visit.

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

The selected image must be Linux x86-64 with glibc, `/bin/sh`, and `cp`. It must contain the language tools and dependencies you want checked.

`lsp-readiness container` remains as a compatibility alias for `check`.

The first check creates `.lsp-readiness/signing.key` with owner-only permissions. Keep that key in your CI secret store if multiple runners must produce packets for the same policy.

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

The website makes no cross-origin request. Its demo uses only bundled sample data.

See [Privacy](https://lsp-readiness-check.sociobot.in/privacy) and [Terms](https://lsp-readiness-check.sociobot.in/terms).

## Develop and verify

Requirements: Rust, Node.js 20 or later, and the preinstalled Playwright Chromium browser.

```sh
npm install
npm test
npm run build
```

`npm test` builds the release CLI and site, runs Rust tests, and runs browser claim and accessibility tests. `npm run build:site` writes the static deploy to `dist/site/`. The full build also places the Linux x86-64 CLI at `dist/site/downloads/lsp-readiness-linux-x86_64`.

Package readiness can be checked without publishing:

```sh
cargo package --allow-dirty
```

## Repository map

- `src/`: CLI library and command.
- `examples/northstar-api/`: bundled demo source.
- `site/`: Vite site and browser demo.
- `tests/`: claim and accessibility tests.
- `.factory/`: brief, design, demo, claims, copy audit, and handoff.

## License

MIT. See [LICENSE](LICENSE).
