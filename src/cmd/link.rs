use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::atlaspack_packages::KNOWN_PACKAGES;

#[derive(Debug, Parser)]
pub struct LinkCommand {}

pub async fn main(
  config: Config,
  _cmd: LinkCommand,
) -> anyhow::Result<()> {
  let node_modules = config.pwd.join("node_modules");
  let node_modules_bin = node_modules.join(".bin");
  let node_modules_atlaspack = node_modules.join("@atlaspack");

  if !fs::exists(&node_modules)? {
    fs::create_dir_all(&node_modules)?;
    fs::create_dir_all(&node_modules_bin)?;
    fs::create_dir_all(&node_modules_atlaspack)?;
  }

  for (name, inner_main) in KNOWN_PACKAGES {
    let package_path = node_modules_atlaspack.join(name);
    let package_json_path = package_path.join("package.json");

    let package_json_bytes = if fs::exists(&package_json_path)? {
      fs::read_to_string(&package_json_path)?
    } else {
      format!(r#"{{ "name": "{}", "main": "./apvm.cjs" }}"#, name).to_string()
    };

    let json::JsonValue::Object(package_json) = json::parse(&package_json_bytes)? else {
      continue;
    };

    let Some(package_name) = package_json.get("main") else {
      continue;
    };

    let package_main = match package_name {
      json::JsonValue::Short(package_main) => package_main.to_string(),
      json::JsonValue::String(package_main) => package_main.clone(),
      _ => continue,
    };

    if package_main == "./apvm.cjs" {
      // continue
    }

    //     let entry_code = format!(r#"
    // if (process.env.APVM_PATH) {{
    //   module.exports = require("{}/{}")
    // }} else {{
    //   module.exports = require("{}")
    // }}
    //     "#, package_main, package_main);
    //     println!("{}", package_main)
  }

  Ok(())
}

/*
// let node_modules_apvm = node_modules.join(".apvm");
  // let node_modules_apvm_old = node_modules_apvm.join("old");
  // let node_modules_apvm_old_atlaspack = node_modules_apvm_old.join("@atlaspack");
  // let node_modules_apvm_old_bin = node_modules_apvm_old.join(".bin");

  if !fs::exists(&node_modules)? {
    fs::create_dir_all(&node_modules)?;
    fs::create_dir_all(&node_modules_bin)?;
    fs::create_dir_all(&node_modules_atlaspack)?;
  }

  // if fs::exists(&node_modules_apvm)? {
  //   return Err(anyhow::anyhow!("Project is already linked"));
  // }

  // fs::create_dir_all(&node_modules_apvm)?;
  // fs::create_dir_all(&node_modules_apvm_old_bin)?;

  // if fs::exists(&node_modules_atlaspack)? {
  //   fs::rename(&node_modules_atlaspack, &node_modules_apvm_old_atlaspack)?;
  // }

  // fs::create_dir_all(&node_modules_atlaspack)?;

  // let target_version = {
  //   if cmd.version == "local" {
  //     if let Some(apvm_local) = config.apvm_local {
  //       apvm_local.clone()
  //     } else {
  //       return Err(anyhow::anyhow!("APVM_LOCAL variable missing"));
  //     }
  //   } else {
  //     let target_version_name = name::encode(&cmd.version)?;
  //     config.apvm_installs_dir.join(target_version_name).join("static")
  //   }
  // };

  // for entry in fs::read_dir(target_version.join("packages"))? {
  //   let entry = entry?;
  //   let entry_path = entry.path();
  //   if !entry_path.is_dir() {
  //     continue;
  //   }
  //   for entry in fs::read_dir(entry_path)? {
  //     let entry = entry?;
  //     let entry_path = entry.path();
  //     if !entry_path.is_dir() {
  //       continue;
  //     }
  //     let entry_package_json = entry_path.join("package.json");
  //     if !entry_package_json.exists() {
  //       continue;
  //     }
  //     let package_json_bytes = fs::read_to_string(entry_package_json)?;
  //     let json::JsonValue::Object(package_json) = json::parse(&package_json_bytes)? else {
  //       continue;
  //     };
  //     let Some(package_name) = package_json.get("name") else {
  //       continue;
  //     };

  //     let package_name = match package_name {
  //       json::JsonValue::Short(package_name) => package_name.to_string(),
  //       json::JsonValue::String(package_name) => package_name.clone(),
  //       _ => continue,
  //     };

  //     if !package_name.starts_with("@atlaspack/") {
  //       continue;
  //     }
  //     let Some(package_name) = package_name.strip_prefix("@atlaspack/") else {
  //       continue;
  //     };
  //     soft_link(&entry_path, &node_modules_atlaspack.join(package_name))?;
  //   }

  //   soft_link(
  //     &target_version,
  //     &node_modules_atlaspack.join("node_modules"),
  //   )?;
  // }

  // println!("✅ Atlaspack Linked ({})", cmd.version);
  // println!("      Source {:?}", target_version);
  // println!("           ➜ {:?}", current_dir);

*/
