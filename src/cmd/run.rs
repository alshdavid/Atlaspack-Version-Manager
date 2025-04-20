use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::atlaspack::atlaspack_exec;

#[derive(Debug, Parser)]
pub struct RunCommand {
  /// Command to run
  pub command: Vec<String>,
}

pub async fn main(
  config: Config,
  cmd: RunCommand,
) -> anyhow::Result<()> {
  todo!();
  // if !fs::exists(&config.apvm_active_dir)? {
  //   return Err(anyhow::anyhow!("No active version installed"));
  // }

  // atlaspack_exec(cmd.command, &config).await?;
  // Ok(())
}
