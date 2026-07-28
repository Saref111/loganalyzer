use std::process::exit;

use crate::error::Result;

mod error;
fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        exit(1);
    }
}

fn run() -> Result<()> {
    Ok(())
}
