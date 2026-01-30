# 0.72.0 (January 26th, 2026)

### Added

- Initial telemetry integration via `fuel-telemetry` crate (enabled via `telemetry` feature)
- Telemetry macros: `info_telemetry!`, `debug_telemetry!`, `warn_telemetry!`, `error_telemetry!`, `trace_telemetry!`, `span_telemetry!`
- Support for `Stdout` tracing writer mode

### Changed

- `init_tracing_subscriber()` now returns `anyhow::Result<Option<WorkerGuard>>` instead of using global static storage
- Log level filter now only applies when explicitly set via CLI flags (previously defaulted to `INFO`)

# 0.71.1 (December 9th, 2025)

### Fixed

- Pin `tracing-subscriber` to 0.3.19 to fix ANSI color output being escaped as literal text

# 0.71.0 (December 8th, 2025)

### Changed

- repo: migrated from `FuelLabs/sway` to `FuelLabs/forc` monorepo

No functional changes from v0.70.2.

# 0.70.2 (November 19th, 2025)

- Previous release from `FuelLabs/sway` repository

For historical changelog entries prior to the migration, see the [sway repository](https://github.com/FuelLabs/sway).
