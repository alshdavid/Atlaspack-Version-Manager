use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::name;

#[derive(Debug, Parser)]
pub struct UninstallCommand {
  /// Target version to uninstall
  pub version: String,
}

pub async fn main(
  config: Config,
  cmd: UninstallCommand,
) -> anyhow::Result<()> {
  let version_safe = name::encode(&cmd.version)?;

  let target = config.apvm_installs_dir.join(version_safe);

  if !target.exists() {
    return Err(anyhow::anyhow!("Not installed",));
  }

  println!("Removing {}", cmd.version);
  fs::remove_dir_all(target)?;

  println!("Removed");

  Ok(())
}
