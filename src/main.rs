use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        Subcommand::Csv(csv_opt) => process_csv(&csv_opt.input, &csv_opt.output),
    }
}
