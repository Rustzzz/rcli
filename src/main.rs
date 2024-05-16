use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, process_decode, process_encode, process_genpass, Opts, Subcommand};

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
        Subcommand::Base64(subcmd) => match subcmd {
            rcli::Base64SubCommand::Encode(opts) => {
                let encode = process_encode(&opts.input, opts.format)?;
                println!("{}",encode);
            },
            rcli::Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                println!("{}",decode);
            },
        },
    }

    Ok(())
}
