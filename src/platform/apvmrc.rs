use std::fs;
use std::path::Path;

use serde::Deserialize;
use toml;

use super::origin::InstallOrigin;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ApvmRc {
  pub specifier: Option<String>,
  pub origin: Option<InstallOrigin>,
}

impl ApvmRc {
  pub fn scan(pwd: &Path) -> anyhow::Result<Option<Self>> {
    let mut current = pwd.to_path_buf();
    loop {
      let config_path = current.join(".apvmrc");
      if fs::exists(&config_path)? {
        let contents = fs::read_to_string(&config_path)?;
        if !contents.contains("=") && contents.contains(".") {
          return Ok(Some(ApvmRc {
            specifier: Some(contents.trim().to_string()),
            origin: Some(InstallOrigin::Super),
          }));
        }
        return Ok(Some(toml::from_str::<ApvmRc>(&contents)?));
      }
      let Some(next) = current.parent() else {
        break;
      };
      current = next.to_path_buf();
    }

    Ok(None)
  }
}
