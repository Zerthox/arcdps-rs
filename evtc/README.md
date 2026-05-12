# evtc

Rust bindings for the ArcDPS EVTC API.
Includes everything shared between Arc's realtime API used by plugins and Arc's log API consumed by parsers.

Documentation can be found at [zerthox.github.io/arcdps-rs/evtc/](https://zerthox.github.io/arcdps-rs/evtc/).

```rs
use evtc::event::{CombatEvent, Event};

fn total_damage_dealt(source: u64, target: u64, events: &[Event]) -> i32 {
    events
        .iter()
        .filter_map(|event| event.try_extract::<CombatEvent>())
        .filter(|event| {
            event.result.is_strike_damage()
                && event.source.id == source
                && event.target.id == target
        })
        .map(|event| event.non_shield_strike_damage())
        .sum()
}
```
