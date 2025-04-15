use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::link::link;
use crate::platform::name;

#[derive(Debug, Parser)]
pub struct UseCommand {
  /// Target version to use
  pub version: String,
}

pub async fn main(
  config: Config,
  cmd: UseCommand,
) -> anyhow::Result<()> {
  let version = cmd.version;

  if version == "local" {
    let Some(apvm_local) = config.apvm_local else {
      return Err(anyhow::anyhow!("$APVM_LOCAL not specified"));
    };
    if config.apvm_active_dir.exists() {
      fs::remove_dir_all(&config.apvm_active_dir)?;
    }
    link(&apvm_local, &config.apvm_active_dir)?;
    println!("Using: local ({})", apvm_local.to_str().unwrap());
    return Ok(());
  }

  let version_safe = name::encode(&version)?;
  let target = config.apvm_installs_dir.join(&version_safe);

  if !target.exists() {
    return Err(anyhow::anyhow!("Not installed"));
  }

  if config.apvm_active_dir.exists() {
    fs::remove_dir_all(&config.apvm_active_dir)?;
  }

  link(&target, &config.apvm_active_dir)?;
  println!("Using: {}", version);
  Ok(())
}
