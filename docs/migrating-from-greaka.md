# Migrating from Greaka's bindings
Besides internal changes some breaking changes and important additions were made to the the API.

The `arcdps_export!` macro had its named shortened to only [`export!`](https://zerthox.github.io/arcdps-bindings/arcdps/macro.export.html).
You can opt to either use `arcdps::export!` or `export!` directly now.

A handful of relevant enums have been added to the crate.
All of them implement a variety of useful traits, for example to convert between them and their primitive numeric counterpart.

[`Agent`](https://zerthox.github.io/arcdps-bindings/arcdps/api/agent/struct.Agent.html) and [`CombatEvent`](https://zerthox.github.io/arcdps-bindings/arcdps/api/event/struct.CombatEvent.html) no longer implement [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) in order to be in line with other structs and avoid accidental implicit duplication.
You can still use [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) as an explicit way to duplicate.

[`CombatEvent`](https://zerthox.github.io/arcdps-bindings/arcdps/api/event/struct.CombatEvent.html) now holds enums in its `iff`, `is_activation`, `is_buff_remove` and `is_statechange` fields rather than primitive numeric values.
[`RawCombatEvent`](https://zerthox.github.io/arcdps-bindings/arcdps/api/event/struct.RawCombatEvent.html) has been added for the raw API and still has the numeric values.

[`Agent`](https://zerthox.github.io/arcdps-bindings/arcdps/api/agent/struct.Agent.html) & [`AgentOwned`](https://zerthox.github.io/arcdps-bindings/arcdps/api/agent/struct.AgentOwned.html) have their `_self` field renamed to `is_self` as a more appropriate name.


Raw structs & types are no longer exported from the root of the crate. You may access them under [`arcdps::api`](https://zerthox.github.io/arcdps-bindings/arcdps/api/) alongside the other structs.

ArcDPS' exports now have proper safe abstractions and are available under [`arcdps::exports`](https://zerthox.github.io/arcdps-bindings/arcdps/exports/).
You can find raw versions of them in [`arcdps::exports::raw`](https://zerthox.github.io/arcdps-bindings/arcdps/exports/raw/).

## Unofficial Extras
Support for [Unofficial Extras](https://github.com/Krappa322/arcdps_unofficial_extras_releases) is hidden behind the `extras` [feature flag](https://doc.rust-lang.org/cargo/reference/features.html) now.
After enabling it, everything specific to Unofficial Extras is available in the [`arcdps::extras`](https://zerthox.github.io/arcdps-bindings/arcdps/extras/) module.

The names of the callbacks have been shortened from for example `unofficial_extras_squad_update` to just `extras_squad_update`.

The `extras_init` function has had its signature changed.
It now receives [`ExtrasAddonInfo`](https://zerthox.github.io/arcdps-bindings/arcdps/extras/struct.ExtrasAddonInfo.html) as a rough equivalent of the struct used in the raw Unofficial Extras API.
The account name of the current player is passed separately as second parameter.
