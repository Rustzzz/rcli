use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, Subcommand};

fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.cmd {
        Subcommand::Csv(csv_opt) => {
            let output = if let Some(output) = csv_opt.output {
                output.clone()
            } else {
                format!("output.{}", csv_opt.format)
            };
            process_csv(&csv_opt.input, &output, csv_opt.format)?
        }
        Subcommand::GenPass(opts) => {
            let password = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{:?}", password);
        }
    }

    Ok(())
}
