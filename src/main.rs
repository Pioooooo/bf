use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use noumisou::{Error, Result};
use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about)]
#[derive(Debug)]
struct Cli {
    program: PathBuf,
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let res = Cli::parse();
    println!("{:?}", res);
    let mut program = BufReader::new(
        File::open(&res.program).map_err(|e| Error::CannottOpenFile(res.program, e))?,
    );
    let mut input: Box<dyn BufRead> = match res.input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(input) => Box::new(BufReader::new(
            File::open(&input).map_err(|e| Error::CannottOpenFile(input, e))?,
        )),
    };
    let mut output: Box<dyn Write> = match res.output {
        None => Box::new(io::stdout()),
        Some(output) => {
            Box::new(File::create(&output).map_err(|e| Error::CannottWriteFile(output, e))?)
        }
    };
    noumisou::run(&mut program, &mut input, &mut output).or_else(|e| {
        println!("{e}");
        Err(e)
    })
}
