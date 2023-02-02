# arcdps_parse

Parsing for ArcDPS EVTC logs.

Documentation can be found at [zerthox.github.io/arcdps-bindings/arcdps_parse/](https://zerthox.github.io/arcdps-bindings/arcdps_parse/).

## Usage
```toml
[dependencies]
arcdps_parse = { git = "https://github.com/zerthox/arcdps-bindings" }
```

A log can be parsed from any input implementing `Read`.
```rs
use arcdps_parse::{Log, Parse};
use std::fs::File;

let mut file = File::open("log.evtc")?;
let log = Log::parse(&mut file)?;
assert_eq!(log.header.revision, 1);
```

Note that ArcDPS may save zipped log files with the `.zevtc` extension.
Reading those can be realized using for example the [zip](https://docs.rs/zip/) crate.
