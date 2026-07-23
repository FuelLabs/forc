# Forc plugins

This repository contains independently released plugins for the Fuel
Orchestrator (`forc`). It does **not** contain the Sway compiler or the core
`forc` executable; those remain in
[`FuelLabs/sway`](https://github.com/FuelLabs/sway).

## Repository split

| Package | Installed commands | Purpose |
|---|---|---|
| `forc-client` | `forc call`, `forc deploy`, `forc run`, `forc submit` | Build and submit transactions to a Fuel node |
| `forc-node` | `forc node` | Start and manage a local Fuel Core node |
| `forc-crypto` | `forc crypto` | Cryptographic operations and conversions |
| `forc-wallet` | `forc wallet` | Generate, import, and manage local wallets |
| `forc-tracing` | internal library | Shared tracing support for Forc plugins |

Compiler-facing commands such as `forc build`, `forc check`, `forc test`,
`forc fmt`, `forc doc`, `forc lsp`, and `forc migrate` are maintained and
released from the Sway repository.

## Choose a compatible toolchain

These packages have independent release numbers. A plugin version should not
be selected by making it numerically match the installed Sway, Forc, Fuel Core,
or SDK version.

For applications targeting a public Fuel network:

1. Install the named `mainnet` or `testnet` toolchain with
   [`fuelup`](https://github.com/FuelLabs/fuelup).
2. Inspect the selected components with `fuelup show` and `forc plugins`.
3. Keep the project's `fuel-toolchain.toml`, `Forc.lock`, SDK lockfile, ABI,
   bytecode, and target chain ID together.
4. Verify command flags with `forc <command> --help` from the selected
   toolchain.

For a custom or source-built toolchain, use
[`releases.toml`](./releases.toml) to see which Sway, Fuel Core, and `fuels-rs`
versions each published plugin release was built against. The file is generated
as part of the release process and is compatibility evidence for that release;
it is not a promise that every other component combination is supported.

## Development

The current workspace dependency set is declared in
[`Cargo.toml`](./Cargo.toml). Run the workspace checks before proposing a
change:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets
cargo test --workspace
```

Release and installation documentation should always name both the plugin
release and the compiler/node versions it was tested with. Avoid the ambiguous
term “latest” unless it is qualified as an upstream release or a specific
Fuelup network channel.
