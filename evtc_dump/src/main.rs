use clap::Parser;
use evtc_parse::parse_file;
use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Parser)]
struct Args {
    /// Input path to EVTC file.
    pub input: String,

    /// Output path to JSON file.
    ///
    /// Defaults to input path with JSON file extension.
    pub output: Option<PathBuf>,
}

impl Args {
    fn output_path(&self) -> PathBuf {
        self.output
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Path::new(&self.input).with_extension("json"))
    }
}

fn main() {
    let args = Args::parse();

    let log = parse_file(&args.input).expect("failed to parse EVTC log");

    let events = log
        .events
        .into_iter()
        .map(|event| event.into_kind())
        .collect::<Vec<_>>();

    let file = File::create(args.output_path()).expect("failed to create output file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &events).expect("failed to write events");
}
