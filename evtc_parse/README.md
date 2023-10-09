# evtc_parse

Parsing for ArcDPS EVTC logs.

Documentation can be found at [zerthox.github.io/arcdps-bindings/evtc_parse/](https://zerthox.github.io/arcdps-bindings/evtc_parse/).

## Usage
```toml
[dependencies]
evtc_parse = { git = "https://github.com/zerthox/arcdps-bindings" }
```

Use the `parse_file` function to easily parse a log from a file path.
```rs
match evtc_parse::parse_file("path/to/log.evtc") {
    Ok(log) => println!("Log for boss id {}", log.header.boss_id),
    Err(err) => eprintln!("Encountered error {}", err),
}
```

A log can also be parsed from any input implementing `Read`.
```rs
use evtc_parse::{Log, Parse};
use std::io;

fn parse_from_read(input: &mut impl io::Read) -> Log {
    Log::parse(input).expect("failed to parse")
}
```

Note that ArcDPS can save compressed log files with `.zevtc` as file extension.
Enabling the `"zevtc"` or `"zip"` feature adds support for compressed logs.

```toml
[dependencies]
evtc_parse = { git = "https://github.com/zerthox/arcdps-bindings", features = ["zevtc"] }
```
