mod cli;
mod process;
pub use process::process_csv;
pub use process::process_genpass;
pub use cli::OutputFormat;
pub use cli::Opts;
pub use cli::Subcommand;
pub use cli::Base64SubCommand;
pub use process::{process_decode,process_encode};
