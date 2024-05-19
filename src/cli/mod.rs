mod base64;
mod csv;
mod genpass;
mod text;
use std::path::Path;
pub use base64::{Base64SubCommand,Base64Format};
use clap::Parser;
pub use csv::OutputFormat;
use self::{csv::CsvOpt, genpass::GenPassOpts};
pub use text::{TextSubCommand,TextSignFormat};

#[derive(Debug, Parser)]
#[command(name = "rcli")]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert Csv to json")]
    Csv(CsvOpt),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not exist!")
    }
}
