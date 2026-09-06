# Landing page copy audit

Audit date: 2026-09-06. Counts treat hyphenated terms, code identifiers, URLs, and numeric tokens as one word. UI labels and code examples are listed separately. No sentence exceeds 22 words and no banned marketing word appears.

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
| The CLI writes one signed JSON readiness report. | 8 | Covered by `signed-packet` |
| Its signature makes tampering detectable (Ed25519). | 6 | Covered by `signed-packet` |
| The normal check uses a network-disabled container made from the exact development image you choose. | 15 | Covered by `local-operation` |
| Detect source languages and declared test commands. | 7 | Product instruction |
| Ignore dependencies, build output, and source contents. | 7 | Product instruction |
| Start each detected language server. | 5 | Product instruction |
| Check formatter versions and run the test command. | 7 | Product instruction |
| Write a signed JSON readiness report. | 7 | Product instruction |
| Verify the report’s signature before an agent starts work. | 9 | Covered by `signed-packet` |
| Use an image address with a SHA-256 digest so the same tools run each time. | 15 | Covered by `local-operation` |
| It does not upload source code or repository file contents. | 10 | Covered by `local-operation` |
| It does not install or update language servers. | 9 | Covered by `no-tool-install` |
| It does not replace your editor, test runner, or container policy. | 11 | Scope statement |
| Subscriptions are not open yet. | 5 | Covered by `subscription-registration-pending` |
| CIAM, GitHub App, and billing registration must pass product QA first. | 11 | Named operator dependencies |
| The free local CLI stays available without an account. | 9 | Covered by `no-account` |

## Direct section names and controls

Section names: `Signed JSON readiness report`, `How the repository check works`, `What the CLI does not do`, and `Private checks for each repository`. Controls: `Try it with sample data`, `Run sample probe`, `Download sample JSON`, `Copy command`, and `Check setup status`. They name their destination or result without slogans or unexplained metaphors.

Pricing fragments: `$49 per repository each month`, `Private CI checks`, `Repository policy templates`, and `Readiness history`. The exact recurring offer and unavailable state are covered by `subscription-registration-pending`.

## Terminology

| Concept | Required term |
| --- | --- |
| One verification run | probe |
| Signed JSON output | readiness report |
| Semantic code process | language server |
| Repository safety state | readiness |
| Browser sample | demo |
| Default isolated operation | repository check |
| Hosted paid work | private CI |
| Account boundary | organization |

## Supporting surfaces

| Surface | Copy | Words | Result |
| --- | --- | ---: | --- |
| Privacy | It reads file names, manifest files, and command output to build a signed JSON readiness report. | 16 | Uses the required term |
| Demo terminal | Sample readiness report is shown only in this demo. | 9 | Uses the required term |
| README | The command creates a temporary readiness report and prints its path. | 10 | Uses the required term |

`packet` remains only in code-level names and JSON schema handling. Visitor-facing output is called a readiness report.

Catalog description: “Verify repository tooling before an agent edits.” It is 48 characters, starts with a verb, and has no marketing words.
