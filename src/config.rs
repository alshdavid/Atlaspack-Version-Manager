use std::path::PathBuf;

use crate::env::Env;
use crate::platform::apvmrc::ApvmRc;
use crate::platform::path_ext::*;

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
  pub pwd: PathBuf,
  pub exe_path: PathBuf,
  pub exe_stem: String,
  pub argv: Vec<String>,
  pub apvm_dir: PathBuf,
  pub apvm_dir_temp: PathBuf,
  pub apvm_installs_dir: PathBuf,
  pub apvm_global_dir: PathBuf,
  pub apvm_runtime: String,
  pub apvm_rc: Option<ApvmRc>,
  // If an APVM_SESSION env var is supplied
  pub session_id: Option<String>,
  pub apvm_active_dir: Option<PathBuf>,
}

impl Config {
  pub fn new(env: &Env) -> anyhow::Result<Self> {
    let apvm_installs_dir = env.apvm_dir.join("versions");
    if !apvm_installs_dir.exists() {
      std::fs::create_dir_all(&apvm_installs_dir)?;
    }

    let mut argv = std::env::args().collect::<Vec<String>>();
    argv.remove(0);

    let exe_path = std::env::current_exe()?;
    let exe_stem = exe_path.try_file_stem()?;

    std::fs::create_dir_all(env.apvm_dir.join("sessions"))?;

    let apvm_install_dir = env
      .apvm_session
      .as_ref()
      .map(|id| env.apvm_dir.join("sessions").join(id));

    let apvm_global_dir = env.apvm_dir.join("global");

    let pwd = std::env::current_dir()?;
    let apvm_rc = ApvmRc::scan(&pwd)?;

    Ok(Self {
      session_id: env.apvm_session.clone(),
      pwd,
      exe_path,
      exe_stem,
      argv,
      apvm_dir: env.apvm_dir.clone(),
      apvm_dir_temp: env.apvm_dir.join(".temp"),
      apvm_installs_dir,
      apvm_global_dir,
      apvm_rc,
      apvm_active_dir: apvm_install_dir,
      apvm_runtime: env.apvm_runtime.clone(),
    })
  }
}
