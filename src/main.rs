mod error;
mod levels;
mod parse;
mod stats;

use std::{io::Read, process::exit};

use crate::{error::Result, parse::LogParser};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        exit(1);
    }
}

fn run() -> Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let mut log_lines = vec![];
    let parser = LogParser::new();

    for line in buf.lines() {
        log_lines.push(parser.parse(line));
    }

    Ok(())
}
