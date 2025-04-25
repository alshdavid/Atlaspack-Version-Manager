use std::fs;
use std::path::PathBuf;

use crate::env::Env;
use crate::platform::active::ActiveVersion;
use crate::platform::apvmrc::ApvmRc;
// use crate::platform::apvmrc::ApvmRc;
use crate::platform::path_ext::*;

#[allow(unused)]
#[derive(Debug)]
pub struct Config {
  pub pwd: PathBuf,
  pub exe_path: PathBuf,
  pub exe_stem: String,
  pub argv: Vec<String>,
  pub apvm_rc: Option<ApvmRc>,
  pub active_version: Option<ActiveVersion>,
  pub paths: Paths,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Paths {
  /// $APVM_DIR (default $HOME/.local/.apvm)
  pub apvm_dir: PathBuf,
  /// $APVM_DIR/.temp
  pub temp: PathBuf,
  /// $APVM_DIR/global
  pub global: PathBuf,
  /// $APVM_DIR/versions
  pub versions: PathBuf,
  /// $APVM_DIR/versions/local
  pub versions_local: PathBuf,
  /// $APVM_DIR/versions/git
  pub versions_git: PathBuf,
  /// $APVM_DIR/versions/npm
  pub versions_npm: PathBuf,
}

impl Config {
  pub fn new(env: &Env) -> anyhow::Result<Self> {
    let pwd = std::env::current_dir()?;

    let mut argv = std::env::args().collect::<Vec<String>>();
    argv.remove(0);

    let exe_path = std::env::current_exe()?;
    let exe_stem = exe_path.try_file_stem()?;

    let apvm_rc = ApvmRc::detect(&pwd)?;

    let apvm_dir = env.apvm_dir.clone();
    let apvm_global_dir = apvm_dir.join("global");
    let apvm_dir_temp = apvm_dir.join(".temp");
    let apvm_versions_dir = apvm_dir.join("versions");
    let apvm_versions_local_dir = apvm_versions_dir.join("local");
    let apvm_versions_git_dir = apvm_versions_dir.join("git");
    let apvm_versions_npm_dir = apvm_versions_dir.join("npm");

    if !fs::exists(&apvm_dir)? {
      fs::create_dir(&apvm_dir)?
    }

    if !fs::exists(&apvm_versions_dir)? {
      fs::create_dir(&apvm_versions_dir)?
    }

    if !fs::exists(&apvm_dir_temp)? {
      fs::create_dir(&apvm_dir_temp)?
    }

    if !fs::exists(&apvm_versions_dir)? {
      fs::create_dir(&apvm_versions_dir)?
    }

    if !fs::exists(&apvm_versions_local_dir)? {
      fs::create_dir(&apvm_versions_local_dir)?
    }

    if !fs::exists(&apvm_versions_git_dir)? {
      fs::create_dir(&apvm_versions_git_dir)?
    }

    if !fs::exists(&apvm_versions_npm_dir)? {
      fs::create_dir(&apvm_versions_npm_dir)?
    }

    let mut config = Self {
      pwd,
      exe_path,
      exe_stem,
      argv,
      apvm_rc,
      active_version: None,
      paths: Paths {
        apvm_dir,
        global: apvm_global_dir,
        temp: apvm_dir_temp,
        versions: apvm_versions_dir,
        versions_local: apvm_versions_local_dir,
        versions_git: apvm_versions_git_dir,
        versions_npm: apvm_versions_npm_dir,
      },
    };

    let active_version = ActiveVersion::parse(&config)?;
    config.active_version = active_version;

    Ok(config)
  }
}
