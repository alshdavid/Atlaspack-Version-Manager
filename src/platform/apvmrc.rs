use std::fs;
use std::path::Path;
use std::path::PathBuf;

use json::JsonValue;

use super::origin::VersionTarget;

#[allow(unused)]
#[derive(Clone, Debug)]
pub struct ApvmRc {
  pub path: PathBuf,
  pub version_target: VersionTarget,
}

impl ApvmRc {
  pub fn detect(pwd: &Path) -> anyhow::Result<Option<Self>> {
    if let Some(apvmrc) = Self::detect_apvmrc(pwd)? {
      return Ok(Some(apvmrc));
    };

    if let Some(apvmrc) = Self::detect_package_json(pwd)? {
      return Ok(Some(apvmrc));
    };

    Ok(None)
  }

  /// Scan and parse ".apvmrc"
  pub fn detect_apvmrc(pwd: &Path) -> anyhow::Result<Option<Self>> {
    let mut current = pwd.to_path_buf();

    loop {
      let config_path = current.join(".apvmrc");
      if fs::exists(&config_path)? {
        let contents = fs::read_to_string(&config_path)?;
        for line in contents.lines() {
          if line.starts_with("#") {
            continue;
          }
          return Ok(Some(Self {
            path: config_path,
            version_target: VersionTarget::parse(line.trim())?,
          }));
        }
        todo!();
      }
      let Some(next) = current.parent() else {
        break;
      };
      current = next.to_path_buf();
    }

    Ok(None)
  }

  /// Scan and parse "package.json#atlaspack.version"
  pub fn detect_package_json(pwd: &Path) -> anyhow::Result<Option<Self>> {
    let mut current = pwd.to_path_buf();

    loop {
      'block: {
        let config_path = current.join("package.json");

        if fs::exists(&config_path)? {
          let contents = fs::read_to_string(&config_path)?;
          let JsonValue::Object(package_json) = json::parse(&contents)? else {
            break 'block;
          };

          let Some(JsonValue::Object(atlaspack)) = package_json.get("atlaspack") else {
            break 'block;
          };

          let specifier = match atlaspack.get("version") {
            Some(JsonValue::String(specifier)) => specifier.clone(),
            Some(JsonValue::Short(specifier)) => specifier.to_string(),
            _ => break 'block,
          };

          return Ok(Some(Self {
            path: config_path,
            version_target: VersionTarget::parse(&specifier)?,
          }));
        }
      }

      let Some(next) = current.parent() else {
        break;
      };
      current = next.to_path_buf();
    }

    Ok(None)
  }
}
