use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::link::hard_link_or_copy;
use crate::platform::link::soft_link;
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

    let target_static = config.apvm_active_dir.join("static");
    let target_bin = config.apvm_active_dir.join("bin");
    let target_lib = config.apvm_active_dir.join("lib");

    fs::create_dir_all(&target_bin)?;
    fs::create_dir_all(&target_lib)?;
    soft_link(&apvm_local, &target_static)?;

    #[cfg(unix)]
    fs::hard_link(config.exe, target_bin.join("atlaspack"))?;

    #[cfg(windows)]
    fs::hard_link(config.exe, target_bin.join("atlaspack.exe"))?;

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
  fs::create_dir_all(&config.apvm_active_dir)?;

  let target_static = config.apvm_active_dir.join("static");
  let target_bin = config.apvm_active_dir.join("bin");
  let target_lib = config.apvm_active_dir.join("lib");

  fs::create_dir_all(&target_bin)?;
  fs::create_dir_all(&target_lib)?;

  hard_link_or_copy(&config.exe_path, &target_bin.join("atlaspack"))?;
  soft_link(&target, &target_static)?;

  println!("Using: {}", version);
  Ok(())
}
