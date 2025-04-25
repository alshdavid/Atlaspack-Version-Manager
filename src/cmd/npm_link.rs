use clap::Parser;

use super::npm_link_npm::npm_link_npm;
use crate::config::Config;
use crate::platform::origin::VersionTarget;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser, Clone)]
pub struct NpmLinkCommand {
  /// Target version to link
  pub version: Option<String>,

  /// Link with support for @atlaspack/* packages
  #[arg(short = 'i', long = "install")]
  pub legacy: bool,
}

pub async fn npm_link(
  config: Config,
  cmd: NpmLinkCommand,
) -> anyhow::Result<()> {
  // Get specifier from CLI or apvm config
  let version = match &cmd.version {
    Some(version) => VersionTarget::try_from(version.as_str())?,
    // Load from config
    None => match &config.active_version {
      Some(active) => active.package.version_target.clone(),
      None => return Err(anyhow::anyhow!("No version selected for install")),
    },
  };

  let package = PackageDescriptor::parse(&config, &version)?;
  if !package.exists()? {
    return Err(anyhow::anyhow!("Version not installed"));
  };

  println!("Linking {}", package.version_target);

  match version {
    VersionTarget::Npm(_) => npm_link_npm(config, cmd, package).await?,
    VersionTarget::Git(_) => npm_link_git(config, cmd, package).await?,
    VersionTarget::Local(_) => npm_link_local(config, cmd, package).await?,
  }

  Ok(())
}

async fn npm_link_git(
  _config: Config,
  _cmd: NpmLinkCommand,
  _package: PackageDescriptor,
) -> anyhow::Result<()> {
  Ok(())
}

async fn npm_link_local(
  _config: Config,
  _cmd: NpmLinkCommand,
  _package: PackageDescriptor,
) -> anyhow::Result<()> {
  Ok(())
}
