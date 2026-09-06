# M2 operator dependencies

No credential belongs in this repository. These registrations must be completed and tested against the deployed product before M2 can be accepted as a hosted account and subscription flow.

## Sociobot Entra CIAM

- Register `https://lsp-readiness-check.sociobot.in/sign-in` as the SPA redirect URL.
- Configure the exact issuer, API audience, public client ID, authorize URL, token URL, JWKS URL, and delegated API scope.
- Add the seven matching `CIAM_*` settings to `sf-lsp-readiness-check-api` through the operator secret path.
- Verify a real first sign-in, repeat sign-in, expired token, wrong audience, wrong issuer, and two-account isolation.

## GitHub App

- Register the setup callback as `https://lsp-readiness-check-api.sociobot.in/api/v1/github/callback`.
- Grant repository metadata read access only for M2. Source/content access is not required.
- Store the app ID, slug, and PEM signing key as server-side settings. The private key must never reach the static site or logs.
- Verify a real approved installation, cancelled installation, expired state, repository selection, installation removal, and cross-organization installation conflict.
- Webhook signing and pull-request check status are M3 work, not an M2 claim.

## Sociobot recurring subscription

- Register `lsp-readiness-check` as a recurring USD subscription at $49 per repository per month.
- Use `https://lsp-readiness-check.sociobot.in/app/billing` as the return URL.
- Supply the supported subscription entitlement and webhook verification contract. The attached one-time license path is not a substitute.
- Verify checkout, return, webhook authenticity, active entitlement, renewal, failed renewal, cancellation, expiry, and refund/revocation in authorized test mode.
- Only after that QA may the product expose a buy action or add `subscription-entitlement` to `.factory/claims.json`.

Public offer metadata is in `.factory/billing-offer.json`. Its `license_validation_path` is intentionally `null` until the recurring contract is supplied.
