mod cli;
mod engine;
mod output_dispatcher;

use crate::cli::ContextCli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = ContextCli::parse();
    engine::execute(args)
}
