use crate::{process_jwt_sign, process_jwt_verify, CmdExcutor};
use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum JwtSubCommand {
    #[command(about = "Sign a payload")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a token with jwt")]
    Verify(JwtVerifyOpts),
}
// rcli jwt sign --sub acme --aud device1 --exp 14d

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long, value_parser = verfiy_len ,help = "Sets the subject (sub) claim")]
    pub sub: String,
    #[arg(short, long, help = "Sets the audience (aud) claim")]
    pub aud: String,
    #[arg(short, long, help = "Sets the expiration time (exp) claim")]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, help = "Sets the token (t) claim")]
    pub t: String,
}

impl CmdExcutor for JwtSubCommand {
    async fn execute(self) -> Result<()> {
        match self {
            JwtSubCommand::Sign(opts) => opts.execute().await,
            JwtSubCommand::Verify(opts) => opts.execute().await,
        }
    }
}

impl CmdExcutor for JwtSignOpts {
    async fn execute(self) -> Result<()> {
        let token = process_jwt_sign(self.sub, self.aud, &self.exp)?;
        println!("{}", token);
        Ok(())
    }
}

impl CmdExcutor for JwtVerifyOpts {
    async fn execute(self) -> Result<()> {
        let is_valid = process_jwt_verify(&self.t)?;
        println!("Jwt is valid: {}", is_valid);
        Ok(())
    }
}

fn verfiy_len(arg: &str) -> Result<String> {
    if arg.len() > 1000 {
        return Err(anyhow::anyhow!("The maximum parameter length is 1000"));
    }
    Ok(arg.into())
}
