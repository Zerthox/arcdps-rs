# ArcDPS bindings

Rust bindings for plugins for [ArcDPS](https://www.deltaconnected.com/arcdps/).
ArcDPS is an addon for [Guild Wars 2](https://guildwars2.com).

This project is originally a fork of [greaka/arcdps_bindings](https://github.com/greaka/arcdps_bindings).

### Features
- Versioning via `Cargo.toml`
- Rust-like abstractions for callbacks and ArcDPS types
- Optional [serde](https://serde.rs/) or [strum](https://docs.rs/strum/latest/strum/) integration
- Imgui interfacing via `imgui-rs`
- [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support
- Optional raw access to ArcDPS C interface

### How to use
```rs
use std::error::Error;
use arcdps::{
    arcdps_export, Agent, CombatEvent, StateChange,
    extras::{UserInfoIter, UserRole},
};

arcdps_export! {
    name: "Example Plugin",
    sig: 123, // change this to a random number
    init,
    combat,
    unofficial_extras_squad_update: squad_update,
}

fn init() -> Result<(), Box<dyn Error>> {
    // may return an error to indicate load failure
    Ok(())
}

fn combat(
    event: Option<&CombatEvent>,
    src: Option<Agent>,
    dest: Option<Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    if let StateChange::EnterCombat = event.is_statechange {
        // source agent has entered combat
    }
}

fn squad_update(users: UserInfoIter) {
    for user in users {
        if let UserRole::SquadLeader | UserRole::Lieutenant = user.role {
            // user can place markers
        }
    }
}
```
