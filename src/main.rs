mod cli;
mod consts;
mod error;
mod levels;
mod parse;
mod report;
mod stats;

use std::{fs, io::Read, process::exit};

use clap::Parser;

use crate::{
    cli::CliArgs, error::Result, parse::LogParser, report::get_report, stats::get_stats_values,
};

fn main() {
    let args = cli::CliArgs::parse();

    if let Err(err) = run(args) {
        eprintln!("{err}");
        exit(1);
    }
}

fn run(args: CliArgs) -> Result<()> {
    let mut buf = String::new();
    if let Some(path) = args.path {
        buf = fs::read_to_string(path)?;
    } else {
        std::io::stdin().read_to_string(&mut buf)?;
    }

    let parser = LogParser::new();

    let stats_values = get_stats_values(buf.lines(), &parser, args.top, args.level);
    let report = get_report(&stats_values, args.format);

    println!("{report}");

    Ok(())
}
