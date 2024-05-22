use anyhow::Result;
use clap::Parser;
use rcli::{CmdExcutor, Opts};
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Opts::parse();
    cli.cmd.execute().await?;
    Ok(())
}
