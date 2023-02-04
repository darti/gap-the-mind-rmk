use clap::{Command, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn main() -> anyhow::Result<()> {
    Ok(())
}
