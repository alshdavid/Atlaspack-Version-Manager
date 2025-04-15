use std::fs;

use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct UninstallCommand {
  /// Target version to uninstall
  pub version: String,
}

pub async fn main(
  config: Config,
  cmd: UninstallCommand,
) -> anyhow::Result<()> {
  let version_safe = urlencoding::encode(&cmd.version).to_string();

  let target = config.apvm_installs_dir.join(version_safe);

  if !target.exists() {
    return Err(anyhow::anyhow!("Not installed",));
  }

  println!("Removing {}", cmd.version);
  fs::remove_dir_all(target)?;

  println!("Removed");

  Ok(())
}
