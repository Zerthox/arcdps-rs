use clap::{Parser, ValueEnum};
use evtc_parse::parse_file;
use serde::Serialize;
use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};
use strum::Display;

#[derive(Debug, Clone, Parser)]
struct Args {
    /// Input path to EVTC file.
    pub input: String,

    /// Output path to JSON file.
    ///
    /// Defaults to input path with JSON file extension.
    pub output: Option<PathBuf>,

    /// Data to dump.
    #[clap(value_enum, long, short, default_value_t)]
    pub data: Data,
}

/// Data to dump.
#[derive(Debug, Display, Default, Clone, ValueEnum)]
#[strum(serialize_all = "lowercase")]
enum Data {
    /// All log data.
    #[default]
    All,

    /// Log agents.
    Agents,

    /// Skill & buff information.
    Skills,

    /// Log events.
    Events,
}

impl Args {
    fn output_path(&self) -> PathBuf {
        self.output
            .as_ref()
            .cloned()
            .unwrap_or_else(|| Path::new(&self.input).with_extension("json"))
    }

    fn save(&self, data: &impl Serialize) {
        let path = self.output_path();
        let file = File::create(&path).expect("failed to create output file");
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &data).expect("failed to write events");
        println!("dumped {} data to \"{}\"", self.data, path.display());
    }
}

fn main() {
    let args = Args::parse();

    let log = parse_file(&args.input)
        .expect("failed to parse EVTC log")
        .into_transformed();

    match args.data {
        Data::All => args.save(&log),
        Data::Agents => args.save(&log.agents),
        Data::Skills => args.save(&log.skills),
        Data::Events => args.save(&log.events),
    }
}
