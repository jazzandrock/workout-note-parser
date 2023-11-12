use std::io::{self, Read, Write};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;
use workout_note_parser::*;

/// Workout notes parser
///
/// Author: Oleg Syniakevych
/// Version: 0.1.0
///
/// Parse workout notes into JSON format.
#[derive(StructOpt, Debug)]
struct Cli {
    /// Input file name or "-" for standard input
    #[structopt(parse(from_os_str), short, long)]
    input: PathBuf,

    /// Output file name, "-" for standard output, default is standard output
    #[structopt(parse(from_os_str), default_value = "-", short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let args = Cli::from_args();

    let mut buffer = String::new();
    if args.input.to_str() == Some("-") {
        io::stdin().read_to_string(&mut buffer)?;
    } else {
        let file = File::open(args.input)?;
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut buffer)?;
    }

    let parsed = parse_workout(&buffer)?;
    let serialized = serde_json::to_string(&parsed)?;

    // Determine the output target
    let mut output: Box<dyn Write> = if args.output.to_str() == Some("-") {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(args.output)?)
    };

    writeln!(output, "{}", &serialized)?;

    Ok(())
}
