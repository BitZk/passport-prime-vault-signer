# Security policy

Vault Signer concerns wallet seed selection and signing. Treat every defect that could expose, substitute, persist, misroute, or misuse seed material as security-sensitive.

## Supported versions

No production version exists. The current repository is a research and UX prototype and must not be used to custody or sign funds.

## Reporting a vulnerability

Do not open a public issue for a vulnerability. Private vulnerability reporting is enabled for this repository. Use:

<https://github.com/BitZk/passport-prime-vault-signer/security/advisories/new>

Include a minimal description, affected commit, impact, and a reproduction that contains no real seed words, private keys, PSBTs, addresses, xpubs, device identifiers, or signing material.

If private reporting is unavailable, contact the maintainer through the private contact method listed on the GitHub profile. Public issues may state only that private contact is needed; do not include technical exploit details.

## If a secret is committed

Deleting the file or rewriting the latest commit is not sufficient because clones, caches, logs, and forks may retain it.

1. Stop using the affected wallet or credential.
2. Move funds or rotate the credential from a known-clean environment.
3. Preserve the commit identifier privately for incident review.
4. Remove the material from all reachable Git history and artifacts.
5. Notify hosting providers and affected collaborators.
6. Document the remediation without reproducing the secret.

Never test this process with a wallet that controls value.

## Prevention limits

GitHub secret scanning and push protection are enabled. The local pre-commit hook and CI also reject selected secret-like filenames and high-confidence private-key patterns without printing matching content. These controls do not recognize every seed phrase or wallet-data format and do not replace manual review before publication. Never place real wallet or signing data in this checkout, even in an ignored file.

## Security gates before any production claim

- Foundation review of the permission and seed-lifecycle design.
- No seed entropy in the navigation protocol, persistent Bitcoin metadata, logs, panic output, crash reports, screenshots, or fixtures.
- Request/response binding and explicit trusted-display confirmation.
- Zeroization on success, cancellation, error, app termination, lock, and shutdown paths.
- Negative tests for spoofing, replay, stale sessions, malformed data, interrupted activation, and wrong-fingerprint signing.
- Existing Bitcoin QR, file, QuantumLink, account, multisig, passphrase, message, address, and PSBT test suites pass unchanged for both primary and temporary seeds.
- Reproducible builds and independent cryptographic/application review.
- Physical-device validation with disposable regtest-only seeds.
