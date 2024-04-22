mod base64;
mod csv;
mod genpass;

use clap::Parser;

pub use self::{
    base64::Base64SubCommand,
    csv::{CsvOpts, OutputFormat},
    genpass::GenPassOpts,
};

#[derive(Debug, Parser)]
#[command(name= "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}
