use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        Subcommand::Csv(csv_opt) => {
            let output = if let Some(output) =csv_opt.output {
                output.clone()
            } else {
               format!("output.{}",csv_opt.format)
            };
            process_csv(&csv_opt.input, &output, csv_opt.format)
        }
    }
}
