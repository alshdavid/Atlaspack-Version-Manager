use std::{fs, os::unix::process};

use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct LinkCommand {}

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
  _config: Config,
  _cmd: LinkCommand,
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
    return Err(anyhow::anyhow!("Project is already linked"))
  }

  fs::create_dir_all(&node_modules_apvm)?;
  fs::create_dir_all(&node_modules_apvm_old_bin)?;

  if fs::exists(&node_modules_atlaspack)? {
    fs::rename(&node_modules_atlaspack, &node_modules_apvm_old_atlaspack)?;
  }
  Ok(())
}
