use std::fs;

use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_key_generate,
    process_sign, process_verify, Opts, Subcommand,
};

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
                println!("{}", encode);
            }
            rcli::Base64SubCommand::Decode(opts) => {
                let decode = process_decode(&opts.input, opts.format)?;
                println!("{}", decode);
            }
        },
        Subcommand::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opt) => process_sign(&opt.input, &opt.key, opt.format)?,
            rcli::TextSubCommand::Verify(opt) => {
                process_verify(&opt.input, &opt.key, opt.format, &opt.sig)?
            }
            rcli::TextSubCommand::Generate(opt) => {
                let key = process_key_generate(opt.format)?;
                match opt.format {
                    rcli::TextSignFormat::Blake3 => {
                        fs::write(opt.output_path.join("blake3.txt"), &key[0])?
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        fs::write(opt.output_path.join("ed25519_public_key.txt"), &key[0])?;
                        fs::write(opt.output_path.join("ed25519_secret_key.txt"), &key[1])?;
                    },
                }
            }
        },
    }

    Ok(())
}
