mod base64;
mod csv;
mod genpass;
mod http;
mod text;
mod jwt;
use self::{csv::CsvOpt, genpass::GenPassOpts, jwt::JwtSubCommand};
use crate::CmdExcutor;
pub use base64::{Base64Format, Base64SubCommand};
use clap::Parser;
pub use csv::OutputFormat;
pub use http::HttpSubCommand;
use std::path::Path;
pub use text::{TextSignFormat, TextSubCommand};

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
    #[command(subcommand, about = "Http Serve")]
    Http(HttpSubCommand),
    #[command(subcommand, about = "Jwt sign/verify")]
    Jwt(JwtSubCommand),
}

pub fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not exist!")
    }
}

impl CmdExcutor for Subcommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Subcommand::Csv(opts) => opts.execute().await,
            Subcommand::GenPass(opts) => opts.execute().await,
            Subcommand::Base64(opts) => opts.execute().await,
            Subcommand::Text(opts) => opts.execute().await,
            Subcommand::Http(opts) => opts.execute().await,
            Subcommand::Jwt(opts) => opts.execute().await,
        }
    }
}
