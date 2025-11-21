# Releasing

This repository is organized as a monorepo containing multiple workspace members. Each crate can be released independently, following a pattern similar to [tokio-rs/tokio](https://github.com/tokio-rs/tokio).

## Release Process

### Tag Naming Convention

Tags follow the format: `{crate-name}-{version}` (no `v` prefix)

Examples:
- `forc-wallet-0.16.0` - Releases the `forc-wallet` crate at version 0.16.0
- `forc-plugin-0.1.0` - Would release a hypothetical `forc-plugin` crate at version 0.1.0

**Note:** Unlike some repositories that use `v1.0.0`, we follow the Tokio convention of no `v` prefix in tags.

### Before Creating a Release

1. **Update the crate version** in `{crate-name}/Cargo.toml`
2. **Update CHANGELOG.md** in the crate directory (`{crate-name}/CHANGELOG.md`)
3. **Commit changes** to the repository
4. **Ensure CI passes** on the main branch

### Creating a GitHub Release

1. Go to the [Releases page](../../releases)
2. Click **"Draft a new release"**
3. In **"Choose a tag"**, type the tag name: `{crate-name}-{version}`
   - Example: `forc-wallet-0.16.0`
   - The tag will be created automatically when you publish
4. Set the **release title** (typically the same as the tag)
5. Add **release notes** (can be based on the CHANGELOG)
6. Click **"Publish release"**

### What Happens Automatically

When you publish a GitHub release with a tag like `forc-wallet-0.16.0`, the CI workflow (`.github/workflows/ci.yml`) automatically triggers two parallel jobs:

#### 1. Publish to crates.io (`publish-crates` job)

- Runs linting and verification checks
- Extracts the crate name from the tag (e.g., `forc-wallet` from `forc-wallet-0.16.0`)
- Verifies the version in `{crate-name}/Cargo.toml` matches the tag version
- Publishes **only that specific crate** to crates.io using the configured registry token
- Only the crate specified in the tag is published; other workspace members are unaffected

#### 2. Build Release Binaries (`build-release` job)

- Extracts the crate name from the tag
- Builds the binary for **only that crate** across multiple platforms:
  - **Linux**: x86_64 and aarch64 (ARM64)
  - **macOS**: x86_64 (Intel) and aarch64 (Apple Silicon)
- Strips debug symbols to reduce binary size
- Creates compressed archives with the naming format: `{tag}-{platform}.tar.gz`
  - Example: `forc-wallet-0.16.0-x86_64-unknown-linux-gnu.tar.gz`
- Uploads all binary archives to the GitHub release

**Key Point:** The CI is fully generic. When you add a new workspace member (e.g., `forc-plugin`) and create a tag `forc-plugin-1.0.0`, the same CI workflow will automatically build and publish that crate without any workflow modifications.

### Release Checklist

- [ ] Version updated in `{crate-name}/Cargo.toml`
- [ ] CHANGELOG.md updated with release notes
- [ ] Changes committed and pushed to main branch
- [ ] CI passing on main branch
- [ ] Tag follows format: `{crate-name}-{version}`
- [ ] Release notes prepared
- [ ] GitHub release created and published

## Workspace Members

Current workspace members:
- `forc-wallet` - A forc plugin for generating or importing wallets using mnemonic phrases

As new crates are added to the workspace, they can be released independently using the same process.
