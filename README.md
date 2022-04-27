# Arcdps Plugin Bindings

This provides arcdps plugin bindings featuring safe, zero-cost abstractions.

Easily integrate into arcdps with just a few lines of code.

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

### How to use
A small example showcasing 2 of the many functions provided.
If `init` returns an error, arcdps won't consider the plugin as loaded and will display the error.
No other function, except for unofficial-extras functions, will be called afterwards.
```rs
use std::error::Error;

use arcdps::UserInfoIter;

arcdps::arcdps_export! {
    name: "example addon",
    sig: 123, // change this to a random number
    unofficial_extras_squad_update: squad_update,
    init: init,
}

fn squad_update(users: UserInfoIter) {
    for user in users.into_iter() {
        println!("{:?}", user);
    }
}

fn init() -> Result<(), Box<dyn Error>> {
    Ok(())
}
```
