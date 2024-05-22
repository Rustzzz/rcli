use anyhow::{Error, Ok};
use clap::Parser;
use std::{fmt, str::FromStr};

use crate::{process_decode, process_encode, CmdExcutor};

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "encode base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "decode base64")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode = process_encode(&self.input, self.format)?;
        println!("{}", encode);
        Ok(())
    }
}

impl CmdExcutor for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode = process_decode(&self.input, self.format)?;
        println!("{}", decode);
        Ok(())
    }
}

impl CmdExcutor for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(opts) => opts.execute().await,
            Base64SubCommand::Decode(opts) => opts.execute().await,
        }
    }
}
