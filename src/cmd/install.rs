use std::fs;

use clap::Parser;

use super::install_git::install_from_git;
use super::install_local::install_from_local;
use super::install_super::install_from_super;
use crate::config::Config;
use crate::platform::origin::InstallOrigin;

#[derive(Debug, Parser)]
pub struct InstallCommand {
  /// Target version to install
  pub version: Option<String>,

  #[arg(short = 'o', long = "origin")]
  pub origin: Option<InstallOrigin>,

  #[arg(short = 'a', long = "alias")]
  pub alias: Option<String>,

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

pub async fn main(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  if cmd.origin.is_none() && cmd.version.is_none() {
    return use_apvm_rc(config, cmd).await;
  }

  fs::create_dir_all(&config.apvm_installs_dir)?;
  fs::create_dir_all(&config.apvm_dir_temp)?;
  fs::create_dir_all(config.apvm_installs_dir.join("git"))?;
  fs::create_dir_all(config.apvm_installs_dir.join("local"))?;
  fs::create_dir_all(config.apvm_installs_dir.join("super"))?;

  match cmd.origin {
    Some(InstallOrigin::Git) => install_from_git(config, cmd).await,
    Some(InstallOrigin::Local) => install_from_local(config, cmd).await,
    Some(InstallOrigin::Super) => install_from_super(config, cmd).await,
    None => install_from_git(config, cmd).await, // None => install_from_super(config, cmd).await
  }
}

async fn use_apvm_rc(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let Some(apvm_rc) = config.apvm_rc.clone() else {
    return Err(anyhow::anyhow!("No version specified"));
  };

  Box::pin(main(
    config,
    InstallCommand {
      version: apvm_rc.specifier,
      origin: Some(apvm_rc.origin),
      ..cmd
    },
  ))
  .await
}
