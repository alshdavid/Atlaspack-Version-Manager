use std::fs;

use super::origin::VersionTarget;
use super::package::PackageDescriptor;
use crate::paths::Paths;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ActiveType {
  Global,
  NodeModules,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ActiveVersion {
  pub active_type: ActiveType,
  pub package: PackageDescriptor,
}

impl ActiveVersion {
  pub fn detect(paths: &Paths) -> anyhow::Result<Option<Self>> {
    // Detect from node_modules
    if let Some(atlaspack_version_path) = &paths.node_modules_atlaspack_version {
      let version = fs::read_to_string(atlaspack_version_path)?;
      return Ok(Some(ActiveVersion {
        active_type: ActiveType::NodeModules,
        package: PackageDescriptor::parse(paths, &VersionTarget::parse(version)?)?,
      }));
    }

    // Select the version from the current config
    // if let Some(apvmrc) = &config.apvm_rc {
    //   let Some(version_target) = &apvmrc.version_target else {
    //     return Ok(None);
    //   };

    //   return Ok(Some(ActiveVersion {
    //     active_type: ActiveType::ProjectConfig,
    //     package: PackageDescriptor::parse(config, version_target)?,
    //   }));
    // }

    // Select the version from the system default
    // TODO
    // if fs::exists(&config.paths.global)? {
    //   let package = PackageDescriptor::parse_from_dir(config, &config.paths.global)?;
    //   return Ok(Some(ActiveVersion {
    //     active_type: ActiveType::Global,
    //     package,
    //   }));
    // }

    Ok(None)
  }
}
