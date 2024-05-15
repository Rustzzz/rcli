use clap::Parser;
use core::fmt;
use std::{path::Path, str::FromStr};

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
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct CsvOpt {
    #[arg(short, long,value_parser = verify_file_exists)]
    pub input: String,
    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
    // default_value 会做一次自动转换 from &str to String
    #[arg(short, long)]
    pub output: Option<String>,
    // default_value_t 不会自动转换，需要和字段类型完全对应
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(file: &str) -> Result<String, &'static str> {
    if Path::new(file).exists() {
        Ok(file.into())
    } else {
        Err("File not exist!")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        let format = match format {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Output format doesnt exist")),
        };
        format
    }
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
