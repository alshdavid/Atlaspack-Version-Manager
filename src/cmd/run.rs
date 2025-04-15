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
  if !fs::exists(&config.apvm_active_dir)? {
    return Err(anyhow::anyhow!("No active version installed"));
  }

  let link = fs::read_link(config.apvm_active_dir.join("static"))?;
  atlaspack_exec(cmd.command, &link, &config).await?;
  Ok(())
}
