use clap::Parser;
use std::path::Path;

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

#[derive(Debug, Parser)]
pub struct CsvOpt {
    #[arg(short, long,value_parser = verify_file_exists)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
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
