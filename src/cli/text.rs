use clap::Parser;
use std::{
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};
use crate::{
    process::{process_decrypt, process_encrypt},
    process_key_generate, process_sign, process_verify, CmdExcutor,
};

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a public/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Encrypt messaga with chacha20poly1305")]
    Encrypt(Chacha20EncryptOpts),
    #[command(about = "Decrypt messaga with chacha20poly1305")]
    Decrypt(Chacha20DecryptOpts),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(KeyGenerateOpts),
}
#[derive(Debug, Parser)]
pub struct Chacha20EncryptOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct Chacha20DecryptOpts {
    #[arg(short, long)]
    pub sig: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_input_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct KeyGenerateOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
    Chacha20poly1305,
}

fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(path);
    if path.exists() && path.is_dir() {
        Ok(path.into())
    } else {
        Err("File does not exist")
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            "chacha20poly1305" => Ok(TextSignFormat::Chacha20poly1305),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
            TextSignFormat::Chacha20poly1305 => "chacha20poly1305",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExcutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sign = process_sign(&self.input, &self.key, self.format)?;
        println!("{}", sign);
        Ok(())
    }
}

impl CmdExcutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let is_verify = process_verify(&self.input, &self.key, self.format, &self.sig)?;
        println!("{}", is_verify);
        Ok(())
    }
}

impl CmdExcutor for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_key_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => fs::write(self.output_path.join("blake3.txt"), &key[0])?,
            TextSignFormat::Ed25519 => {
                fs::write(self.output_path.join("ed25519_public_key.txt"), &key[0])?;
                fs::write(self.output_path.join("ed25519_secret_key.txt"), &key[1])?;
            }
            TextSignFormat::Chacha20poly1305 => {
                fs::write(self.output_path.join("chacha20.key"), &key[0])?;
            }
        }
        Ok(())
    }
}

impl CmdExcutor for Chacha20EncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let process_encrypt = process_encrypt(&self.input, &self.key)?;
        println!("{}", process_encrypt);
        Ok(())
    }
}

impl CmdExcutor for Chacha20DecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypt = process_decrypt(&self.sig, &self.key)?;
        println!("{}", decrypt);
        Ok(())
    }
}

impl CmdExcutor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opt) => opt.execute().await,
            TextSubCommand::Verify(opt) => opt.execute().await,
            TextSubCommand::Generate(opt) => opt.execute().await,
            TextSubCommand::Encrypt(opt) => opt.execute().await,
            TextSubCommand::Decrypt(opt) => opt.execute().await,
        }
    }
}
