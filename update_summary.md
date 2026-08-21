# Documentation update summary

## Why this update was needed

At the 2026-07-23 recheck, the public Forc command reference
(docs.fuel.network/docs/forc/) still published a single "Stable" Forc 0.69.0
and obscured the independent release cadence of the network-facing plugins
that now live in this repository. That can make a Sway compiler version look
like an authoritative version for unrelated plugin binaries.

## What changed

- Expanded the repository README from a placeholder into an ownership map.
- Listed the command groups provided by `forc-client`, `forc-node`,
  `forc-crypto`, `forc-wallet`, and `forc-tracing`.
- Explained that core Forc, the compiler, and in-tree development plugins
  remain in the Sway repository.
- Documented independent plugin versioning and the purpose of `releases.toml`.
- Recommended named Fuelup network channels for deployed-network
  compatibility rather than choosing components by whichever release number
  looks newest.
- Added verification guidance using `fuelup show`, `forc plugins`, and
  `forc <command> --help` from the selected toolchain.
- Warned against the unqualified term "latest" unless it is qualified as an
  upstream release or a specific Fuelup network channel.

## Validation

- Cargo metadata passed with the repository's pinned Rust toolchain.
- The documented packages and installed binary names were checked against the
  workspace manifests.
- All committed changes pass `git diff --check`.
