# evtc

Rust bindings for the ArcDPS EVTC API.
Includes everything shared between Arc's realtime API used by plugins and Arc's log API consumed by parsers.

Documentation can be found at [zerthox.github.io/arcdps-rs/evtc/](https://zerthox.github.io/arcdps-rs/evtc/).

```rs
use evtc::Event;

fn total_damage_dealt(source: u64, target: u64, events: &[Event]) -> i32 {
    events
        .iter()
        .filter_map(|event| event.try_to_strike())
        .filter(|strike_event| {
            strike_event.strike.dealt_damage()
                && strike_event.source.id == source
                && strike_event.target.id == target
        })
        .map(|strike_event| strike_event.total_damage - strike_event.shield_damage as i32)
        .sum()
}
```
