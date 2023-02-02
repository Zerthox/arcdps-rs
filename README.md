# ArcDPS bindings

[Rust](https://rust-lang.org) bindings for [ArcDPS](https://www.deltaconnected.com/arcdps/).
ArcDPS is an addon for [Guild Wars 2](https://guildwars2.com).


This project is split into multiple crates covering different parts of ArcDPS.

|||
|---|---|
| [arcdps](./arcdps) | Bindings for ArcDPS plugins.
| [arcdps_evtc](./arcdps_evtc) | Bindings for the ArcDPS EVTC API.
| [arcdps_parse](./arcdps_parse) | Parsing for ArcDPS EVTC logs.

If you are interested in writing an ArcDPS plugin in Rust, take a look at the [arcdps](./arcdps) crate in this repository.
Its documentation can be found at [zerthox.github.io/arcdps-bindings/arcdps/](https://zerthox.github.io/arcdps-bindings/arcdps/).
Parts of this project are originally a fork of [greaka/arcdps_bindings](https://github.com/greaka/arcdps_bindings).
