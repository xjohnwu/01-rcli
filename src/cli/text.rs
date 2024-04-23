use super::{verify_file, verify_path};
use clap::Parser;
use core::fmt;
use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign text with a private key.")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message.")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key.")]
    Generate(TextKeyGenerateOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, default_value="blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextKeyGenerateOpts {
    #[arg(long, default_value="blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(long, default_value="blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(short, long)]
    pub sig: String,
}

fn parse_format(s: &str) -> Result<TextSignFormat, String> {
    match s {
        "blake3" => Ok(TextSignFormat::Blake3),
        "ed25519" => Ok(TextSignFormat::Ed25519),
        _ => Err(format!("Invalid format: {}", s)),
    }
}

impl FromStr for TextSignFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_format(s)
    }
}

impl From<TextSignFormat> for String {
    fn from(f: TextSignFormat) -> String {
        match f {
            TextSignFormat::Blake3 => "blake3".to_string(),
            TextSignFormat::Ed25519 => "ed25519".to_string(),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}
