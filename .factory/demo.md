# Demo sandbox

- Website: `https://lsp-readiness-check.sociobot.in/demo` or local `/demo`.
- CLI: `lsp-readiness demo`.
- Sample: `northstar-api`, a TypeScript and Rust repository with two fixture language servers, two fixture formatters, and 42 passing tests.
- Reset: choose **Reset demo** in the website banner. The CLI creates a new directory under the operating system temporary directory for each process.
- Browser storage namespace: `demo:lsp-readiness-check`. Demo mode never reads or writes any other application namespace.
- Source fixture: [`examples/northstar-api`](../examples/northstar-api). `lsp-readiness demo` executes this trusted fixture's TAP tests and bundled tools through the inspection engine used inside a normal check's sandbox. The website replay and downloadable packet are a signed record of that probe.

The website demo can be verified without an account or network request outside the product origin. The CLI prints the temporary packet path so tests can inspect and verify it.
