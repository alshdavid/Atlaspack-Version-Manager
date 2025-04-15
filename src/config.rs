use std::path::PathBuf;

use crate::ApvmCommand;

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
  pub argv: Vec<String>,
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
      argv: std::env::args().skip(1).collect::<Vec<String>>(),
      apvm_dir,
      apvm_installs_dir,
      apvm_install_dir,
      apvm_local: cmd.apvm_local.clone(),
    })
  }
}

fn apvm_dir_default() -> anyhow::Result<PathBuf> {
  let Ok(Some(current_exe)) = homedir::my_home() else {
    return Err(anyhow::anyhow!("Cannot find apvm_home. Please set $APVM_HOME variable manually"))
  };
  let default_dir = current_exe.join(".local").join("apvm").join("apvm_dir");
  if default_dir.is_file() {
    return Err(anyhow::anyhow!("{:?} exists but is a file", current_exe));
  }
  if !default_dir.exists() {
    std::fs::create_dir_all(&default_dir)?;
  }
  Ok(default_dir)
}
