use std::fs;

use super::package::PackageDescriptor;
use crate::config::Config;

#[allow(unused)]
#[derive(Debug)]
pub enum ActiveType {
  Global,
  ProjectConfig,
  Override,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ActiveVersion {
  pub active_type: ActiveType,
  pub package: PackageDescriptor,
}

impl ActiveVersion {
  pub fn parse(config: &Config) -> anyhow::Result<Option<Self>> {
    // Select the version from the current config
    if let Some(apvmrc) = &config.apvm_rc {
      return Ok(Some(ActiveVersion {
        active_type: ActiveType::ProjectConfig,
        package: PackageDescriptor::parse(config, &apvmrc.version_target)?,
      }));
    }

    // Select the version from the system default
    if fs::exists(&config.paths.global)? {
      let package = PackageDescriptor::parse_from_dir(config, &config.paths.global)?;
      return Ok(Some(ActiveVersion {
        active_type: ActiveType::Global,
        package,
      }));
    }

    Ok(None)
  }

  pub fn exists(&self) -> anyhow::Result<bool> {
    self.package.exists()
  }
}
