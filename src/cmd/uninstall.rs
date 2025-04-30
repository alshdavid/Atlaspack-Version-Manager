use std::fs;

use clap::Parser;

use crate::context::Context;
use crate::platform::origin::VersionTarget;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser)]
pub struct UninstallCommand {
  /// Target version to uninstall
  pub version: String,
}

pub fn main(
  ctx: Context,
  cmd: UninstallCommand,
) -> anyhow::Result<()> {
  let version_target = VersionTarget::parse(&cmd.version)?;
  let package = PackageDescriptor::parse(&ctx, &version_target)?;

  if !package.exists()? {
    return Err(anyhow::anyhow!("Not installed",));
  }

  println!("Removing {}", cmd.version);
  fs::remove_dir_all(package.path)?;

  println!("Removed");

  Ok(())
}
