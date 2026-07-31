use std::path::PathBuf;

use clap::Parser;

use crate::{levels::Level, report::Format};

#[derive(Parser)]
pub struct CliArgs {
    pub path: Option<PathBuf>,

    #[arg(long, default_value_t = Format::Text)]
    pub format: Format,

    #[arg(long, default_value_t = 5)]
    pub top: usize,

    #[arg(long)]
    pub level: Option<Level>,
}
