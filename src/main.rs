use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author = "maniacHanz")]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show csv, or convert csv to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    output: String,

    #[arg(short, long, default_value_t = ',')] // '' is char, "" is str
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // import module
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err(format!("Input file '{}' does not exist.", filename))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
