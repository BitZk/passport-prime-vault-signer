# Development

## Toolchain

Foundation's public SDK is Nix-first and supports macOS and Linux. Follow the [official Get Started guide](https://docs.foundation.xyz/developers/get-started/) rather than installing an unrelated Rust toolchain.

As observed on 2026-08-25, the developer pages still displayed “1.0.0-beta.1” while `https://sdk.foundation.xyz/latest/install.sh` selected SDK `1.0.0`. The release installer verifies both GPG signatures and SHA-256 checksums. Treat the installer and documentation as moving inputs and record the version used in every validation report.

```bash
curl -sSfL https://sdk.foundation.xyz/latest/install.sh | bash
foundation develop
foundation doctor
```

The current repository was structured from the signed `multi-page-app` template in SDK 1.0.0. Its minimum target is KeyOS 1.3.1 because that public source includes imported Bitcoin seeds in Vault.

## Simulator workflow

Inside `foundation develop`:

```bash
foundation clean
foundation preview ui/app.slint
foundation sim
cargo test
```

Run `foundation clean` after an SDK update so generated themes, manifests, and UI mappings match the new bundle.

The simulator does not prove secure-element behavior, zeroization, peripheral behavior, or power-loss handling. Record simulator and hardware results separately.

The dependency-free protocol state model can also be tested with an ordinary Rust compiler, independently of the SDK:

```bash
rustc --edition=2021 --test src/protocol.rs -o /tmp/vault-signer-protocol-tests
/tmp/vault-signer-protocol-tests
```

CI runs this host-only check and records the compiler version. It is not a KeyOS or cryptographic test.

## Signing and hardware

The public repository intentionally omits `signing-identity` and `cosign2-config` from `app-config.toml`. Foundation's documentation says these values are machine-specific and should not be committed.

Do not generate signing material simply to work on the UX prototype. When a hardware milestone is explicitly approved:

1. Generate the publisher identity outside the repository with `foundation cert gen`.
2. Back up the private key through a separately reviewed secret-management process.
3. Publish only the certificate fingerprint needed by users.
4. Register the certificate on a disposable development device.
5. Sideload only a build produced from a reviewed commit.

Never use a production wallet seed during development. Hardware tests must use disposable regtest-only seeds generated for that test run.

## Source snapshots

For upstream work, clone KeyOS into a separate clean worktree and pin the exact full commit SHA. Do not copy an unreviewed moving branch into this repository. Each test report must include:

- Foundation SDK version and archive checksum;
- KeyOS full SHA and tag;
- host OS and architecture;
- commands run and exact result;
- simulator or device boundary;
- device firmware version and developer certificate fingerprint, when applicable;
- confirmation that public-tree checks passed.

## Repository checks

`scripts/check-public-tree.sh` rejects common secret/signing filenames and high-confidence private-key content. It is a backstop, not proof that a commit is safe.

`scripts/check-spdx.sh` ensures code, configuration, and scripts retain license identifiers.

Always inspect the staged diff manually:

```bash
git diff --cached --check
git diff --cached
```
