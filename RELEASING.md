# Releasing

This repository is organized as a monorepo containing multiple workspace members. Each crate can be released independently, following a pattern similar to [tokio-rs/tokio](https://github.com/tokio-rs/tokio).

## Release Process

### Tag Naming Convention

Tags follow the format: `{crate-name}-{version}`

Examples:
- `forc-wallet-0.16.0` - Releases the `forc-wallet` crate at version 0.16.0
- `forc-plugin-0.1.0` - Would release a hypothetical `forc-plugin` crate at version 0.1.0

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

### What Happens Next

The CI workflow (`.github/workflows/ci.yml`) will automatically:

1. **Parse the tag** to extract crate name and version
2. **Verify** the version in `{crate-name}/Cargo.toml` matches the tag
3. **Build release binaries** for multiple platforms (Linux x64/ARM64, macOS x64/ARM64)
4. **Publish to crates.io** with the configured registry token
5. **Upload binaries** to the GitHub release

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
