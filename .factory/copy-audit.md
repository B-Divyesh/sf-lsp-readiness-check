# Landing page copy audit

Audit date: 2026-09-02. Counts treat hyphenated terms, code identifiers, URLs, and numeric tokens as one word. UI labels and code examples are listed separately. No sentence exceeds 22 words and no banned marketing word appears.

## First screen

| Copy | Words | Result |
| --- | ---: | --- |
| Verify tooling before an agent edits | 6 | Pass; job-first headline |
| For teams onboarding contributors who need code navigation, diagnostics, formatting, and tests ready before changes begin. | 16 | Pass |
| See a finished probe in one click. | 7 | Pass |
| Source stays on your machine | 5 | Covered by `local-operation` |
| The demo reloads offline after its first visit | 8 | Covered by `offline-demo` |
| No account is needed for the free CLI | 8 | Covered by `no-account` |

Read-aloud check: “Verify tooling before an agent edits. Try it with sample data to see a finished probe.” It states the job and first action in one breath.

## Landing sections

| Copy | Words | Result |
| --- | ---: | --- |
| The CLI writes one JSON packet. | 6 | Covered by `signed-packet` |
| It records each probe, the repository inventory digest, and an Ed25519 signature. | 12 | Covered by `signed-packet` |
| The normal check runs in a network-disabled container made from your digest-pinned development image. | 14 | Covered by `local-operation` |
| Detect source languages and declared test commands. | 7 | Product instruction |
| Ignore dependencies, build output, and source contents. | 7 | Product instruction |
| Start each detected language server. | 5 | Product instruction |
| Check formatter versions and run the test command. | 7 | Product instruction |
| Write a JSON capability packet. | 5 | Product instruction |
| Verify its Ed25519 signature before an agent starts work. | 9 | Covered by `signed-packet` |
| It does not upload source code or repository file contents. | 10 | Covered by `local-operation` |
| It does not install or update language servers. | 9 | Covered by `no-tool-install` |
| It does not replace your editor, test runner, or container policy. | 11 | Scope statement |

## Direct section names and controls

Section names: `Signed capability packet`, `How the repository check works`, and `What the CLI does not do`. Controls: `Try it with sample data`, `Run sample probe`, `Download sample JSON`, and `Copy command`. They name their destination or result without slogans or unexplained metaphors.

## Terminology

| Concept | Required term |
| --- | --- |
| One verification run | probe |
| Signed JSON output | capability packet |
| Semantic code process | language server |
| Repository safety state | readiness |
| Browser sample | demo |
| Default isolated operation | repository check |

Catalog description: “Verify repository tooling before an agent edits.” It is 46 characters, starts with a verb, and has no marketing words.
