# ArcDPS Rust bindings

[Rust](https://rust-lang.org) bindings for [ArcDPS](https://www.deltaconnected.com/arcdps/).
ArcDPS is an addon for [Guild Wars 2](https://guildwars2.com).

This project is split into multiple crates covering different parts of ArcDPS.

| Crate | Purpose |
|---|---|
| [arcdps](./arcdps) | Bindings for ArcDPS plugins.
| [evtc](./evtc) | Bindings for the ArcDPS EVTC API.
| [evtc_dump](./evtc_dump) | CLI tool to dump ArcDPS EVTC log contents as JSON.
| [evtc_parse](./evtc_parse) | Parsing for ArcDPS EVTC logs.

If you are interested in writing an ArcDPS plugin in Rust, take a look at the [arcdps](./arcdps) crate in this repository.
Its documentation can be found at [zerthox.github.io/arcdps-rs/arcdps/](https://zerthox.github.io/arcdps-rs/arcdps/).
Parts of this project are originally a fork of [greaka/arcdps_bindings](https://github.com/greaka/arcdps_bindings).
