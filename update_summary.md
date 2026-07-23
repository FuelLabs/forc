# Documentation update summary

## Why this update was needed

Public Forc documentation often presents core Forc and every plugin as one
versioned tool even though network-facing plugins moved into this repository
and now release independently. That can make a Sway compiler version look like
an authoritative version for unrelated plugin binaries.

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
- Added verification guidance using `forc --version`, plugin `--version` and
  `--help`, `fuelup show`, and the selected network manifest.
- Warned against presenting ambiguous "latest Forc" documentation without
  identifying the executable, source repository, version, and target network.

## Validation

- Cargo metadata passed with the repository's pinned Rust toolchain.
- The documented packages and installed binary names were checked against the
  workspace manifests.
- All committed changes pass `git diff --check`.
