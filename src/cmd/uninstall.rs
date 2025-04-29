use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::origin::VersionTarget;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser)]
pub struct UninstallCommand {
  /// Target version to uninstall
  pub version: String,
}

pub fn main(
  config: Config,
  cmd: UninstallCommand,
) -> anyhow::Result<()> {
  let version_target = VersionTarget::parse(&cmd.version)?;
  let package = PackageDescriptor::parse(&config, &version_target)?;

  if !package.exists()? {
    return Err(anyhow::anyhow!("Not installed",));
  }

  println!("Removing {}", cmd.version);
  fs::remove_dir_all(package.path)?;

  println!("Removed");

  Ok(())
}
