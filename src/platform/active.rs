use std::fs;
use std::path::PathBuf;

use super::name;
use super::origin::InstallOrigin;
use super::path_ext::*;
use crate::config::Config;

#[allow(unused)]
pub struct ActivePackage {
  pub kind: InstallOrigin,
  pub name_encoded: String,
  pub name: String,
  pub real_path: PathBuf,
  pub link_path: PathBuf,
  pub static_path: PathBuf,
}

impl ActivePackage {
  pub fn new(config: &Config) -> anyhow::Result<Option<Self>> {
    let target_path = config.apvm_active_dir.join("static");
    if !fs::exists(&target_path)? {
      return Ok(None);
    }
    let link = fs::read_link(config.apvm_active_dir.join("static"))?;
    let link_name_encoded = link.try_file_name()?;
    let link_name = name::decode(&link_name_encoded)?;
    let link_kind = link.try_parent()?.try_file_name()?;
    Ok(Some(Self {
      kind: InstallOrigin::try_from(link_kind)?,
      name_encoded: link_name_encoded,
      name: link_name,
      real_path: link,
      link_path: config.apvm_active_dir.clone(),
      static_path: target_path.clone(),
    }))
  }
}
