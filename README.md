# ArcDPS bindings

Rust bindings for [ArcDPS](https://www.deltaconnected.com/arcdps/) plugins.
ArcDPS is an addon for [Guild Wars 2](https://guildwars2.com).

Documentation can be found at [zerthox.github.io/arcdps-bindings/arcdps/](https://zerthox.github.io/arcdps-bindings/arcdps/).

This project is originally a fork of [greaka/arcdps_bindings](https://github.com/greaka/arcdps_bindings).

## Features
- Rust abstractions for ArcDPS callbacks, types and exports
- ImGui interfacing via [imgui-rs](https://github.com/imgui-rs/imgui-rs)
- Versioning via `Cargo.toml`
- Optional [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support
- Optional logging via [log](https://github.com/rust-lang/log)
- Optional [serde](https://serde.rs/) and [strum](https://docs.rs/strum/latest/strum/) integration
- Optional access to raw C interface of ArcDPS

## Usage
```toml
[dependencies]
arcdps = { git = "https://github.com/zerthox/arcdps-bindings" }
```

```rs
use std::error::Error;
use arcdps::{Agent, CombatEvent, StateChange};

arcdps::export! {
    name: "Example Plugin",
    sig: 0x12345678, // change this to a random number
    init,
    combat: custom_combat_name,
}

fn init() -> Result<(), Box<dyn Error>> {
    // may return an error to indicate load failure
    Ok(())
}

fn custom_combat_name(
    event: Option<CombatEvent>,
    src: Option<Agent>,
    dst: Option<Agent>,
    skill_name: Option<&str>,
    id: u64,
    revision: u64,
) {
    if let Some(event) = event {
        if let StateChange::EnterCombat = event.is_statechange {
            // source agent has entered combat
        }
    }
}
```

## Unofficial Extras
[Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) support is hidden behind the `extras` feature flag.

```toml
[dependencies.arcdps]
git = "https://github.com/zerthox/arcdps-bindings"
features = ["extras"]
```

```rs
use arcdps::extras::{UserInfoIter, UserRole};

arcdps::export! {
    name: "Example Plugin",
    sig: 0x12354678,
    extras_squad_update,
}

fn extras_squad_update(users: UserInfoIter) {
    for user in users {
        if let UserRole::SquadLeader | UserRole::Lieutenant = user.role {
            // user can place markers
        }
    }
}
```
