use std::fs::File;
use std::io::{self, BufRead};

use clap::Parser;

mod parse;
use crate::parse::*;

mod exec;
use crate::exec::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to a `simply` script to run
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = File::open(args.path)?;
    let lines = io::BufReader::new(file).lines();
    let mut instructions = Vec::new();
    for line in lines {
        let instruction = Instruction::try_from(line?)?;
        instructions.push(instruction);
    }

    let mut program = Program::default();
    program.run(instructions)?;

    Ok(())
}
