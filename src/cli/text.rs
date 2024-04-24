use crate::CmdExecutor;

use super::{verify_file, verify_path};
use clap::Parser;
use core::fmt;
use std::{path::PathBuf, str::FromStr};
use tokio::fs;

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

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let signed = crate::process_text_sign(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let verified = crate::process_text_verify(&self.input, &self.key, self.format, &self.sig)?;
        if verified {
            println!("Signature verified");
        } else {
            println!("Signature not verified");
        }
        Ok(())
    }
}

impl CmdExecutor for TextKeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = crate::process_text_generate(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let name = self.output.join("blake3.key");
                fs::write(name, &key[0]).await?;
            }
            TextSignFormat::Ed25519 => {
                let name = &self.output;
                fs::write(name.join("ed25519.sk"), &key[0]).await?;
                fs::write(name.join("ed25519.pk"), &key[1]).await?;
            }
        }
        Ok(())
    }
}

impl CmdExecutor for TextSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
        }
    }
}
