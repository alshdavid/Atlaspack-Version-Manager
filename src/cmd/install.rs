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
  pub version: String,

  #[arg(short = 'o', long = "origin", default_value = "super")]
  pub origin: InstallOrigin,

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
  fs::create_dir_all(&config.apvm_installs_dir)?;
  fs::create_dir_all(&config.apvm_dir_temp)?;
  fs::create_dir_all(config.apvm_installs_dir.join("git"))?;
  fs::create_dir_all(config.apvm_installs_dir.join("local"))?;
  fs::create_dir_all(config.apvm_installs_dir.join("super"))?;

  match cmd.origin {
    InstallOrigin::Git => install_from_git(config, cmd).await,
    InstallOrigin::Local => install_from_local(config, cmd).await,
    InstallOrigin::Super => install_from_super(config, cmd).await,
  }
}
