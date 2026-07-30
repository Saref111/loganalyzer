mod consts;
mod error;
mod levels;
mod parse;
mod report;
mod stats;

use std::{io::Read, process::exit};

use crate::{
    error::Result,
    parse::LogParser,
    report::{ReportType, get_report},
    stats::get_stats_values,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        exit(1);
    }
}

fn run() -> Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let parser = LogParser::new();

    let stats_values = get_stats_values(buf.lines(), &parser, None);
    let report_json = get_report(&stats_values, ReportType::Json);
    let report_text = get_report(&stats_values, ReportType::Text);

    println!("{report_json}");
    println!("{report_text}");

    Ok(())
}
