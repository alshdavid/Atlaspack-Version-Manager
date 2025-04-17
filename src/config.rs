use std::path::PathBuf;

use rand::Rng;
use rand::distr::Alphanumeric;

use crate::env::Env;
use crate::platform::path_ext::*;

#[derive(Debug)]
pub struct Config {
  pub id: String,
  pub exe_path: PathBuf,
  pub exe_stem: String,
  pub argv: Vec<String>,
  pub apvm_dir: PathBuf,
  pub apvm_dir_temp: PathBuf,
  pub apvm_installs_dir: PathBuf,
  pub apvm_active_dir: PathBuf,
  pub apvm_runtime: String,
}

impl Config {
  pub fn new(cmd: &Env) -> anyhow::Result<Self> {
    let id = match &cmd.apvm_session {
      Some(id) => id.clone(),
      None => rand::rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect::<String>(),
    };

    let apvm_installs_dir = cmd.apvm_dir.join("versions");
    if !apvm_installs_dir.exists() {
      std::fs::create_dir_all(&apvm_installs_dir)?;
    }

    let mut argv = std::env::args().collect::<Vec<String>>();
    argv.remove(0);

    let exe_path = std::env::current_exe()?;
    let exe_stem = exe_path.try_file_stem()?;

    std::fs::create_dir_all(cmd.apvm_dir.join("sessions"))?;
    let apvm_install_dir = cmd.apvm_dir.join("sessions").join(&id);

    Ok(Self {
      id,
      exe_path,
      exe_stem,
      argv,
      apvm_dir: cmd.apvm_dir.clone(),
      apvm_dir_temp: cmd.apvm_dir.join(".temp"),
      apvm_installs_dir,
      apvm_active_dir: apvm_install_dir,
      apvm_runtime: cmd.apvm_runtime.clone(),
    })
  }
}
