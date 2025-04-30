use std::fs;
use std::time::SystemTime;

use clap::Parser;

use super::install_local::install_from_local;
use crate::cmd::install_git::install_from_git;
use crate::cmd::install_npm::install_from_npm;
use crate::context::Context;
use crate::platform::origin::VersionTarget;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser)]
pub struct InstallCommand {
  /// Target version to install
  pub version: Option<String>,

  /// Replace an existing version if already installed
  #[arg(short = 'f', long = "force")]
  pub force: bool,

  /// Skips any build steps
  #[arg(long = "skip-build")]
  pub skip_build: bool,

  /// Forward stdout/stderr for the underlying commands
  #[arg(short = 'v', long = "verbose")]
  pub verbose: bool,
}

pub fn main(
  ctx: Context,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let start_time = SystemTime::now();

  // Get specifier from CLI or apvm config
  let version = match &cmd.version {
    Some(version) => VersionTarget::try_from(version.as_str())?,
    // Load from config
    None => match &ctx.active_version {
      Some(active) => active.package.version_target.clone(),
      None => return Err(anyhow::anyhow!("No version selected for install")),
    },
  };

  let package = PackageDescriptor::parse(&ctx.paths, &version)?;
  let exists = package.exists()?;

  if exists && !cmd.force {
    println!("✅ Already installed");
    return Ok(());
  }

  if exists {
    println!("Removing Existing");
    fs::remove_dir_all(&package.path)?;
  }

  // dbg!(&config);
  // dbg!(&cmd);
  // dbg!(&package);

  match &version {
    VersionTarget::Npm(_) => install_from_npm(ctx, cmd, package)?,
    VersionTarget::Git(_) => install_from_git(ctx, cmd, package)?,
    VersionTarget::Local(_) => install_from_local(ctx, cmd, package)?,
  };

  println!(
    "✅ Installed in {:.2?} ({})",
    start_time.elapsed()?,
    version
  );
  Ok(())
}
