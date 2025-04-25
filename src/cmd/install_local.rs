use std::path::PathBuf;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::link;
use crate::platform::package::PackageDescriptor;

pub async fn install_from_local(
  _config: Config,
  _cmd: InstallCommand,
  package: PackageDescriptor,
) -> anyhow::Result<()> {
  // For local packages the version is the path to the package
  let original_path = PathBuf::from(package.version);

  println!("Linking");
  link::soft_link(&original_path, &package.path)?;

  Ok(())
}
