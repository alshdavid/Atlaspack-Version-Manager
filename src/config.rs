use std::path::PathBuf;

use rand::Rng;
use rand::distr::Alphanumeric;

use crate::env::Env;

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
  pub id: String,
  pub exe: String,
  pub argv: Vec<String>,
  pub apvm_dir: PathBuf,
  pub apvm_installs_dir: PathBuf,
  pub apvm_active_dir: PathBuf,
  pub apvm_local: Option<PathBuf>,
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
    let exe = PathBuf::from(argv.remove(0))
      .file_stem()
      .unwrap()
      .to_str()
      .unwrap()
      .to_string();

    std::fs::create_dir_all(cmd.apvm_dir.join("sessions"))?;
    let apvm_install_dir = cmd.apvm_dir.join("sessions").join(&id);

    Ok(Self {
      id,
      exe,
      argv,
      apvm_dir: cmd.apvm_dir.clone(),
      apvm_installs_dir,
      apvm_active_dir: apvm_install_dir,
      apvm_local: cmd.apvm_local.clone(),
    })
  }
}
