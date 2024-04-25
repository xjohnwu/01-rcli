use crate::CmdExecutor;

use super::{verify_file, verify_path};
use clap::Parser;
use core::fmt;
use enum_dispatch::enum_dispatch;
use std::{path::PathBuf, str::FromStr};
use tokio::fs;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign text with a private key.")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signed message.")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a new key.")]
    Generate(TextKeyGenerateOpts),
    #[command(about = "Encrypt a message.")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt a message.")]
    Decrypt(TextDecryptOpts),
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
    ChaCha20Poly1305,
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

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
}

fn parse_format(s: &str) -> Result<TextSignFormat, String> {
    match s {
        "blake3" => Ok(TextSignFormat::Blake3),
        "ed25519" => Ok(TextSignFormat::Ed25519),
        "chacha20poly1305" => Ok(TextSignFormat::ChaCha20Poly1305),
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
            TextSignFormat::ChaCha20Poly1305 => "chacha20poly1305".to_string(),
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
            TextSignFormat::ChaCha20Poly1305 => {
                let name = self.output.join("chacha20poly1305.key");
                fs::write(name, &key[0]).await?;
            }
        }
        Ok(())
    }
}

impl CmdExecutor for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encrypted = crate::process_text_encrypt(&self.input, &self.key)?;
        println!("{}", encrypted);
        Ok(())
    }
}

impl CmdExecutor for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decrypted = crate::process_text_decrypt(&self.input, &self.key)?;
        println!("{}", decrypted);
        Ok(())
    }
}
