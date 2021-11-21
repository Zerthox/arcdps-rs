# Arcdps Rust Wrapper

This is a WIP arcdps Rust Wrapper featuring safe abstractions where possible and sane.

### Features

Current features include:
- Versioning plugins via Cargo.toml
- A simple interface for all callbacks
- Optional opt out of safe abstractions to directly access the arcdps C interface
- Imgui interfacing via `imgui-rs`
- Logging to arcdps via the `log` crate
- [unofficial extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) bindings

Still in development:
- Exposing settings from arcdps

Still exploring technical boundaries:
- Arcdps-like snapping of imgui windows

### Documentation

You can build the documentation by invoking `cargo docs --open` for now.

If you have any questions, please contact me or create a PR to improve it.
