mod cli;
mod process;
mod utils;
pub use process::process_csv;
pub use process::process_genpass;
pub use process::process_sign;
pub use process::process_verify;
pub use process::process_key_generate;
pub use process::{process_decode,process_encode};
pub use process::process_http_serve;
pub use cli::OutputFormat;
pub use cli::Opts;
pub use cli::Subcommand;
pub use cli::Base64SubCommand;
pub use cli::TextSubCommand;
pub use cli::TextSignFormat;
pub use cli::HttpSubCommand;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExcutor {
    async fn execute(self) -> anyhow::Result<()>;
}
