use anyhow::{anyhow, Result};
use clap::Parser;

use ormat::cli::{run_enumerate, Action, Args};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Enum => run_enumerate(),
        _ => Err(anyhow!("Not implemented")),
    }
}
