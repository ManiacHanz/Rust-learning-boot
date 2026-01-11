use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author = "maniacHanz")]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show csv, or convert csv to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    #[arg(short, long, default_value_t = ',')] // '' is char, "" is str
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

pub fn verify_input_file(filename: &str) -> Result<String, String> {
    // import module
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("Input file '{}' does not exist.", filename))
    }
}
