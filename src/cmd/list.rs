use std::fs;

use clap::Parser;

use crate::{config::Config, platform::name};

#[derive(Debug, Parser)]
pub struct ListCommand {}

pub async fn main(
  config: Config,
  _cmd: ListCommand,
) -> anyhow::Result<()> {
  println!("Atlaspack Versions Installed:");

  let mut active_name = String::default();
  if fs::exists(&config.apvm_install_dir)? {
    'main: {
      let link = fs::read_link(&config.apvm_install_dir)?;

      if let Some(apvm_local) = &config.apvm_local {
        if link == *apvm_local {
          active_name = apvm_local.to_str().unwrap().to_string();
          break 'main;
        }
      }
      
      if let Some(basename) = link.file_name() {
        if let Some(basename) = basename.to_str() {
          active_name =  name::decode(basename.to_string())?
        }
      }
    }
  }

  if let Some(apvm_local) = config.apvm_local {
    let apvm_local = apvm_local.to_str().unwrap().to_string();
    if apvm_local == active_name {
      println!("  ✅ local 🍝 ({})", apvm_local);
    } else {
      println!("  ⏸️  local 🍝 ({})", apvm_local);
    }
  }

  for entry in fs::read_dir(&config.apvm_installs_dir)? {
    let entry = entry?;
    let file_name = entry.file_name();
    let Some(file_name) = file_name.to_str() else {
      return Err(anyhow::anyhow!("Unable to render version"));
    };
    let file_name = name::decode(&file_name)?;

    if file_name == active_name {
      println!("  ✅ {}", file_name);
    } else {
      println!("  ⏸️  {}", file_name);
    }
  }
  Ok(())
}
