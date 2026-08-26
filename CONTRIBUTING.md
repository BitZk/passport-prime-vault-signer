# Contributing

Thank you for helping make Vault Signer safer and easier to review.

## Before opening a change

1. Read the [project status and resume criteria](docs/foundation-status.md), [Architecture](docs/architecture.md), [Threat model](docs/threat-model.md), and decisions under `docs/adr/`. Internal ADR acceptance does not mean Foundation approval.
2. Search existing issues and discussions.
3. While implementation is paused, limit contributions to scoped non-secret UX/model work, documentation, and test proposals. A design issue does not authorize seed handling, permission changes, cryptography integration, or hardware work; those require the documented resume gates first.
4. Use only synthetic metadata. Never include a mnemonic, private key, real PSBT, xpub, address, device identifier, certificate, or signing configuration.

## Local checks

From a Foundation SDK development shell:

```bash
foundation doctor
foundation sim
cargo test
```

From any ordinary shell:

```bash
git config core.hooksPath .githooks
./scripts/check-public-tree.sh
./scripts/check-spdx.sh
```

The local pre-commit hook scans the Git index, not just the working tree, so a staged secret cannot be hidden by editing the file after staging. CI repeats the staged-tree check.

Document any check you could not run. A simulator result is not a hardware result, and a successful build is not a security review.

## Pull requests

- Keep each pull request focused on one reviewable change.
- Explain the threat-model impact and failure behavior.
- Add tests for success, rejection, cancellation, malformed input, and stale/replayed requests when applicable.
- Preserve Foundation copyright and GPL notices in any KeyOS-derived patch.
- Do not add sensitive permissions to `app-config.toml` as a workaround for the platform boundary.
- Do not commit generated SDK links, build artifacts, signing identities, or local machine paths.

All commits must include a Developer Certificate of Origin sign-off:

```text
Signed-off-by: Your Name <your-email@example.com>
```

Use `git commit -s` to add it.
