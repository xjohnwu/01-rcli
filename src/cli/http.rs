use std::path::PathBuf;

use super::verify_path;
use clap::Parser;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Send a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}
