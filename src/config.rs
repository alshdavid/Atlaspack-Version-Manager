use std::path::PathBuf;

use crate::ApvmCommand;

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
  pub apvm_dir: PathBuf,
  pub apvm_installs_dir: PathBuf,
  pub apvm_install_dir: PathBuf,
  pub apvm_local: Option<PathBuf>,
}

impl Config {
  pub fn new(cmd: &ApvmCommand) -> anyhow::Result<Self> {
    let apvm_dir = match &cmd.apvm_dir {
      Some(apvm_dir) => apvm_dir.clone(),
      None => apvm_dir_default()?,
    };

    let apvm_installs_dir = apvm_dir.join("versions");
    if !apvm_installs_dir.exists() {
      std::fs::create_dir_all(&apvm_installs_dir)?;
    }

    let apvm_install_dir = apvm_dir.join("active");

    Ok(Self {
      apvm_dir,
      apvm_installs_dir,
      apvm_install_dir,
      apvm_local: cmd.apvm_local.clone(),
    })
  }
}

fn apvm_dir_default() -> anyhow::Result<PathBuf> {
  let current_exe = std::env::current_exe()?;
  let Some(exe_dir) = current_exe.parent() else {
    return Err(anyhow::anyhow!("{:?} has no directory", current_exe));
  };
  let default_dir = exe_dir.join("apvm_dir");
  if default_dir.is_file() {
    return Err(anyhow::anyhow!("{:?} exists but is a file", current_exe));
  }
  if !default_dir.exists() {
    std::fs::create_dir_all(&default_dir)?;
  }
  Ok(default_dir)
}
