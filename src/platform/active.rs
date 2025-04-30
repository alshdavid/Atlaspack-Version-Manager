use super::package::PackageDescriptor;
use crate::env::Env;

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum ActiveType {
  Global,
  ProjectConfig,
  NpmLink,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct ActiveVersion {
  pub active_type: ActiveType,
  pub package: PackageDescriptor,
}

impl ActiveVersion {
  pub fn detect(_env: &Env) -> anyhow::Result<Option<Self>> {
    // Detect from node_modules

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
