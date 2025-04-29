use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::link;
use crate::platform::origin::VersionTarget;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser)]
pub struct DefaultCommand {
  /// Target version to use
  pub version: String,
}

pub fn main(
  config: Config,
  cmd: DefaultCommand,
) -> anyhow::Result<()> {
  let version_target = VersionTarget::parse(&cmd.version)?;
  let package = PackageDescriptor::parse(&config, &version_target)?;

  if !package.exists()? {
    return Err(anyhow::anyhow!("Version not installed"));
  }

  if fs::exists(&config.paths.global)? {
    fs::remove_dir_all(&config.paths.global)?;
  }

  link::soft_link(&package.path_real()?, &config.paths.global)?;

  println!("✅ Default version set: {}", version_target);

  Ok(())
}
