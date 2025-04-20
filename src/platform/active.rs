use std::fs;
use std::path::Path;
use std::path::PathBuf;

use super::name;
use super::origin::InstallOrigin;
use super::path_ext::*;
use crate::config::Config;

#[derive(Debug)]
pub enum ActivePackageKind {
  Session,
  Global,
}

#[allow(unused)]
#[derive(Debug)]
pub struct ActivePackage {
  pub kind: ActivePackageKind,
  pub origin: InstallOrigin,
  pub name_encoded: String,
  pub name: String,
  pub session_path: PathBuf,
  pub static_path: PathBuf,
  pub static_path_real: PathBuf,
}

impl ActivePackage {
  pub fn active_or_global(config: &Config) -> anyhow::Result<Option<Self>> {
    Ok(Some(match ActivePackage::active(config)? {
      Some(active) => active,
      None => match ActivePackage::global(config)? {
        Some(active) => active,
        None => return Err(anyhow::anyhow!("No active package selected")),
      },
    }))
  }

  pub fn try_active_or_global(config: &Config) -> anyhow::Result<Self> {
    Ok(match ActivePackage::active(config)? {
      Some(active) => active,
      None => match ActivePackage::global(config)? {
        Some(active) => active,
        None => return Err(anyhow::anyhow!("No active package selected")),
      },
    })
  }

  pub fn active(config: &Config) -> anyhow::Result<Option<Self>> {
    let apvm_active_dir = match &config.apvm_active_dir {
      Some(apvm_active_dir) => apvm_active_dir,
      None => return Ok(None),
    };

    if !fs::exists(apvm_active_dir)? {
      return Ok(None);
    }

    let target_path = apvm_active_dir.join("static");
    if !fs::exists(&target_path)? {
      return Ok(None);
    }
    Self::resolve(apvm_active_dir, ActivePackageKind::Session)
  }

  pub fn global(config: &Config) -> anyhow::Result<Option<Self>> {
    if !fs::exists(&config.apvm_global_dir)? {
      return Ok(None);
    }

    Self::resolve(&config.apvm_global_dir, ActivePackageKind::Global)
  }

  fn resolve(
    target_dir: &Path,
    kind: ActivePackageKind,
  ) -> anyhow::Result<Option<Self>> {
    let session_path = target_dir.to_path_buf();
    let static_path = target_dir.join("static");
    let static_path_real = fs::read_link(&static_path)?;
    let name_encoded = static_path_real.try_file_name()?;
    let name = name::decode(&name_encoded)?;
    let origin = InstallOrigin::try_from(static_path_real.try_parent()?.try_file_name()?)?;

    let static_path_real = match origin {
      InstallOrigin::Super => static_path_real,
      InstallOrigin::Git => static_path_real,
      InstallOrigin::Local => fs::read_link(&static_path_real)?,
    };

    Ok(Some(Self {
      kind,
      origin,
      name_encoded,
      name,
      session_path,
      static_path,
      static_path_real,
    }))
  }
}
