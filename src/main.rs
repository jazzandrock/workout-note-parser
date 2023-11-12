use serde_json;
use std::io::{self, Read};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;
use workout_note_parser::*;

#[derive(StructOpt, Debug)]
struct Cli {
    /// Input file, or '-' to read from stdin
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

    let args = Cli::from_args();

    let mut buffer = String::new();
    if let Some(input) = args.input.as_ref() {
        if input.to_str() == Some("-") {
            io::stdin().read_to_string(&mut buffer)?;
        } else {
            let file = File::open(input)?;
            let mut reader = BufReader::new(file);
            reader.read_to_string(&mut buffer)?;
        }
    }

    let input = r#"
        Name of the first exercise
        20 x 10 This is a comment. There you write 
        30 x 10 how you felt during the exercise,
        40 x 10 like "this was close to the edge"
        50 x 10 or "this was easy, better increase the weight".
        60 x 10 The first number is the weight, the second is the number of reps.
        70 x 10 + 40 x 6 Sometimes you do all you can with one weight and then 
        80 x 10 immediately you take a smaller weight and do a few more reps. 
        90 x 10 You can write it as well

        bench press
        20 x 10
        50 x 10
        60 x 10 near death experience
        70 x 5 + 40 x 10   
    "#;

    let vec = parse_workout(input)?;

    let output = serde_json::to_string_pretty(&vec)?;
    println!("{}", &output);

    Ok(())
}
