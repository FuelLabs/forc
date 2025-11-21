# 0.16.1 (November 21st, 2025)

### Changed

- deps: added fuels to the workspace dependencies

# 0.16.0 (November 21st, 2025)

### Changed

- deps: bump tokio from 1.44.1 to 1.44.2 ([#252])
- deps: bump tracing-subscriber from 0.3.19 to 0.3.20 ([#251])
- ci: swap deprecated actions/upload-release-assets to shogo82148/actions-upload-release-asset ([#224])

### Fixed

- wallet: fix typo throughout codebase ([#253])
- wallet: fix typo in Wallet Accounts Directory Path ([#247])

[#224]: https://github.com/FuelLabs/forc-wallet/pull/224
[#247]: https://github.com/FuelLabs/forc-wallet/pull/247
[#251]: https://github.com/FuelLabs/forc-wallet/pull/251
[#252]: https://github.com/FuelLabs/forc-wallet/pull/252
[#253]: https://github.com/FuelLabs/forc-wallet/pull/253

# 0.15.2 (October 6th, 2025)

### Changed

- deps: upgraded Rust stable from 1.86.0 to 1.90.0
- deps: updated fuels dependency from 0.74 to 0.75

# 0.15.1 (June 19th, 2025)

### Changed

- deps: updated forc-tracing from 0.67.0 to 0.68.8

# 0.15.0 (June 12th, 2025)

### Added

- wallet: introduced wallet export capability

### Changed

- deps: updated fuels-rs dependencies

# 0.14.0 (April 23rd, 2025)

### Changed

- deps: upgraded fuels to 0.72 and fuel-core to 0.43.1
- deps: updated Rust to 1.86

# 0.13.0 (March 24th, 2025)

### Changed

- refactor: refactored and updated fuels and other dependencies

# 0.12.0 (January 22nd, 2025)

### Changed

- wallet: removed beta networks
- deps: upgraded SDK to 0.70

# 0.11.1 (October 30th, 2024)

### Fixed

- wallet: fixed minor wallet issues

### Changed

- refactor: improved struct visibility

# 0.11.0 (October 15th, 2024)

### Added

- wallet: implemented checksum reporting

### Changed

- wallet: updated default target

# 0.10.0 (October 5th, 2024)

### Changed

- deps: upgraded to fuels v0.66.6

# 0.9.1 (September 23rd, 2024)

### Added

- wallet: added plain address printing by default

# 0.9.0 (August 12th, 2024)

### Changed

- deps: bump to sdk 0.66

# 0.8.2 (July 9th, 2024)

### Added

- ci: add code coverage reports

### Changed

- deps: update all dependencies
- deps: bump to use sdk 0.65

# 0.8.1 (June 24th, 2024)

### Added

- wallet: derive and cache some accounts with forc-wallet import

### Changed

- deps: bump sdk to 0.64.0 to support fuel-core v0.28.0
- ci: use FuelLabs fork of publish-crates

# 0.8.0 (May 31st, 2024)

### Added

- wallet: point to testnet for default query end point

### Changed

- deps: bump to fuel-core v0.27.0

# 0.7.1 (May 16th, 2024)

### Changed

- deps: bump fuels-rs to 0.62.0

# 0.7.0 (May 8th, 2024)

### Changed

- deps: use rust-sdk v0.60 to support fuel-core v0.26.0

# 0.6.1 (April 26th, 2024)

### Changed

- deps: bump fuel-core to v0.24.3

# 0.6.0 (April 12th, 2024)

### Added

- wallet: add `forc wallet list` command

### Changed

- deps: bump to latest sdk for fuel-core 0.24.2

# 0.5.0 (March 28th, 2024)

### Changed

- deps: bump to sdk 0.56.0, vm 0.47

# 0.4.3 (February 15th, 2024)

### Added

- wallet: point to beta-5 by default

### Changed

- refactor: use common forc utils for formatting error & warning output

# 0.4.2 (January 4th, 2024)

### Added

- wallet: expose new options for new_cli so that it can be used outside this repo

# 0.4.1 (January 4th, 2024)

### Added

- wallet: added `--force` option to forc-wallet new command

### Changed

- deps: SDK version upgraded to 0.54.0

# 0.4.0 (November 30th, 2023)

### Added

- wallet: tx-policies support

### Changed

- deps: bumped fuels-rs to 0.49, 0.50.1, and 0.53.0
- wallet: updated error command messaging

# 0.3.0 (August 28th, 2023)

### Changed

- wallet: point to beta-4 faucet and block explorer by default

# 0.2.5 (August 11th, 2023)

### Changed

- deps: upgrade to fuel-core v0.20.1

# 0.2.4 (July 17th, 2023)

### Added

- wallet: expose balance collecting and printing functionality

### Changed

- deps: bumped fuels, fuels-core, and fuel dependencies

### Fixed

- wallet: enforce 0x prefix for hex data input
- wallet: fixed forc-wallet balance command panic post-clap v4

# 0.2.3 (July 5th, 2023)

### Added

- wallet: added transfer functionality
- wallet: expose cli functionality as a lib
- wallet: expose derive_secret_key functionality

### Changed

- deps: bumped clap to v4.2.4
- deps: bumped fuels-* to 0.43 and fuel-* to 0.31
- wallet: updated password input messaging

### Fixed

- wallet: fixed plain address queries with as-hex flag

# 0.2.2 (March 30th, 2023)

### Added

- wallet: added `balance` and `account <ix> balance` subcommands
- wallet: added beta-3 network URL; switched to beta-3 as default
- wallet: added plain address derivation support

### Changed

- docs: only print EXAMPLES section on long help

# 0.2.1 (February 22nd, 2023)

### Added

- wallet: added support for displaying public key for derived accounts

# 0.2.0 (February 21st, 2023)

### Changed

This release contains breaking changes to the CLI interface:

- **Breaking**: `forc wallet init` � `forc wallet new`
- **Breaking**: `forc wallet new` � `forc wallet account new`
- **Breaking**: `forc wallet list` � `forc wallet accounts`
- **Breaking**: Removed `--account-index` flag in favor of subcommands
- **Breaking**: Accounts command now requires password verification; added `--unverified` flag
- **Breaking**: Sign command restructured with subcommands (tx-id, string, file, hex)

# 0.1.3 (January 24th, 2023)

### Added

- wallet: new signing capability using private keys for local testing

### Changed

- refactor: testing enhancements and separation of CLI from core logic
- refactor: import functionality converted from argument to standalone command
- ci: added markdown linting to CI pipeline
- deps: Rust 1.65 and 1.66.1 compatibility fixes

### Documented

- docs: documentation updates for export/import features
- docs: README quickstart improvements

# 0.1.2 (October 1st, 2022)

### Fixed

- wallet: fixed error handling for non-existent vault paths

### Changed

- refactor: refactored string utility functions

# 0.1.1 (September 29th, 2022)

### Added

- wallet: HD wallet regeneration from mnemonic phrases
- wallet: password-based phrase input (no command parameters)
- wallet: separate import command implementation
- wallet: account private key export functionality

### Changed

- ci: CI binary building integration
- deps: fuels SDK version updates to 0.24.0
- refactor: separated export to dedicated command
- wallet: SDK derivation path integration

# 0.1.0 (August 29th, 2022)

Initial release featuring core wallet operations, HD wallet support, account management, signing functionality, and SDK integration.

### Added

- wallet: HD wallet support with BIP-39 mnemonic phrases
- wallet: account derivation and management
- wallet: transaction signing functionality
- wallet: integration with fuels SDK
