use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::link::soft_link;
use crate::platform::name;

#[derive(Debug, Parser)]
pub struct LinkCommand {
  /// What version to link
  pub version: String,
}

/*
  /node_modules
    /.apvm
      /active -> $(apvm list --only-active)
      /old
        /.bin
          atlaspack
        /@atlaspack
          ...
    /.bin
      /atlaspack -> ../.apvm/atlaspack
    /@atlaspack
      ... -> ../.apvm/active/...
*/

pub async fn main(
  config: Config,
  cmd: LinkCommand,
) -> anyhow::Result<()> {
  let current_dir = std::env::current_dir()?;
  let node_modules = current_dir.join("node_modules");
  let node_modules_bin = node_modules.join(".bin");
  let node_modules_apvm = node_modules.join(".apvm");
  let node_modules_apvm_old = node_modules_apvm.join("old");
  let node_modules_apvm_old_atlaspack = node_modules_apvm_old.join("@atlaspack");
  let node_modules_apvm_old_bin = node_modules_apvm_old.join(".bin");
  let node_modules_atlaspack = node_modules.join("@atlaspack");

  if !fs::exists(&node_modules)? {
    fs::create_dir_all(&node_modules)?;
    fs::create_dir_all(&node_modules_bin)?;
    fs::create_dir_all(&node_modules_atlaspack)?;
  }

  if fs::exists(&node_modules_apvm)? {
    return Err(anyhow::anyhow!("Project is already linked"));
  }

  fs::create_dir_all(&node_modules_apvm)?;
  fs::create_dir_all(&node_modules_apvm_old_bin)?;

  if fs::exists(&node_modules_atlaspack)? {
    fs::rename(&node_modules_atlaspack, &node_modules_apvm_old_atlaspack)?;
  }

  fs::create_dir_all(&node_modules_atlaspack)?;

  let target_version = {
    if cmd.version == "local" {
      if let Some(apvm_local) = config.apvm_local {
        apvm_local.clone()
      } else {
        return Err(anyhow::anyhow!("APVM_LOCAL variable missing"));
      }
    } else {
      let target_version_name = name::encode(&cmd.version)?;
      config.apvm_installs_dir.join(target_version_name).join("static")
    }
  };

  for entry in fs::read_dir(target_version.join("packages"))? {
    let entry = entry?;
    let entry_path = entry.path();
    if !entry_path.is_dir() {
      continue;
    }
    for entry in fs::read_dir(entry_path)? {
      let entry = entry?;
      let entry_path = entry.path();
      if !entry_path.is_dir() {
        continue;
      }
      let entry_package_json = entry_path.join("package.json");
      if !entry_package_json.exists() {
        continue;
      }
      let package_json_bytes = fs::read_to_string(entry_package_json)?;
      let json::JsonValue::Object(package_json) = json::parse(&package_json_bytes)? else {
        continue;
      };
      let Some(package_name) = package_json.get("name") else {
        continue;
      };

      let package_name = match package_name {
        json::JsonValue::Short(package_name) => package_name.to_string(),
        json::JsonValue::String(package_name) => package_name.clone(),
        _ => continue,
      };

      if !package_name.starts_with("@atlaspack/") {
        continue;
      }
      let Some(package_name) = package_name.strip_prefix("@atlaspack/") else {
        continue;
      };
      soft_link(&entry_path, &node_modules_atlaspack.join(package_name))?;
    }

    soft_link(
      &target_version,
      &node_modules_atlaspack.join("node_modules"),
    )?;
  }

  println!("✅ Atlaspack Linked ({})", cmd.version);
  println!("      Source {:?}", target_version);
  println!("           ➜ {:?}", current_dir);
  Ok(())
}
