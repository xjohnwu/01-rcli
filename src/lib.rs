mod cli;
mod process;
mod utils;

pub use cli::Opts;
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExectuor {
    async fn execute(self) -> anyhow::Result<()>;
}
