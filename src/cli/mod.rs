mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

use clap::Parser;

use crate::CmdExecutor;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::{CsvOpts, OutputFormat},
    genpass::GenPassOpts,
    http::HttpSubCommand,
    text::{TextSignFormat, TextSubCommand},
};

#[derive(Debug, Parser)]
#[command(name= "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode or decode base64")]
    Base64(Base64SubCommand),
    #[command(subcommand, about = "Text sign/verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Serve a directory over HTTP")]
    Http(HttpSubCommand),
}

impl CmdExecutor for SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            SubCommand::Csv(opts) => opts.execute().await?,
            SubCommand::GenPass(opts) => opts.execute().await?,
            SubCommand::Base64(subcmd) => subcmd.execute().await?,
            SubCommand::Text(subcmd) => subcmd.execute().await?,
            SubCommand::Http(subcmd) => subcmd.execute().await?,
        }
        Ok(())
    }
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    // if input is "-" or file exists
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File not found")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path not found or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File not found"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exist"), Err("File not found"));
    }
}
