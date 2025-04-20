use std::fs;
use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;
use crate::platform::link;
use crate::platform::name;
use crate::platform::origin::InstallOrigin;
use crate::platform::path_ext::PathExt;

#[derive(Debug, Parser)]
pub struct UseCommand {
  /// Target version to use
  pub version: Option<String>,

  /// Global version
  #[arg(short = 'g', long = "global")]
  pub global: bool,

  #[arg(short = 'o', long = "origin", default_value = "super")]
  pub origin: Option<InstallOrigin>,
}

pub async fn main(
  config: Config,
  cmd: UseCommand,
) -> anyhow::Result<()> {
  match cmd.origin {
    Some(InstallOrigin::Super) => use_super(config, cmd).await,
    Some(InstallOrigin::Git) => use_git(config, cmd).await,
    Some(InstallOrigin::Local) => use_local(config, cmd).await,
    None => todo!(),
  }
}

async fn use_super(
  config: Config,
  cmd: UseCommand,
) -> anyhow::Result<()> {
  let Some(version) = &cmd.version else {
    return Err(anyhow::anyhow!("Version not installed"));
  };
  let version_safe = name::encode(version)?;

  let installs_dir = config.apvm_installs_dir.join("super");
  let target_dir = installs_dir.join(&version_safe);

  let use_dir = get_use_dir(&config, &cmd)?;

  let target_static = use_dir.join("static");
  let target_bin = use_dir.join("bin");

  if !fs::exists(&target_dir)? {
    return Err(anyhow::anyhow!("Version not installed"));
  }

  if fs::exists(&use_dir)? {
    fs::remove_dir_all(&use_dir)?;
  }
  fs::create_dir_all(&use_dir)?;
  fs::create_dir_all(&target_bin)?;

  link::hard_link_or_copy(&config.exe_path, &target_bin.join("atlaspack"))?;
  link::soft_link(&target_dir, &target_static)?;

  println!("Using: {} (git)", version);
  Ok(())
}

async fn use_git(
  config: Config,
  cmd: UseCommand,
) -> anyhow::Result<()> {
  let version = cmd.version.clone().unwrap_or("main".to_string());
  let version_safe = name::encode(&version)?;

  let installs_dir = config.apvm_installs_dir.join("git");
  let target_dir = installs_dir.join(&version_safe);
  let use_dir = get_use_dir(&config, &cmd)?;

  let target_static = use_dir.join("static");
  let target_bin = use_dir.join("bin");

  if !fs::exists(&target_dir)? {
    return Err(anyhow::anyhow!("Version not installed"));
  }

  if fs::exists(&use_dir)? {
    fs::remove_dir_all(&use_dir)?;
  }
  fs::create_dir_all(&use_dir)?;
  fs::create_dir_all(&target_bin)?;

  link::hard_link_or_copy(&config.exe_path, &target_bin.join("atlaspack"))?;
  link::soft_link(&target_dir, &target_static)?;

  println!("Using: {} (git)", version);
  Ok(())
}

async fn use_local(
  config: Config,
  cmd: UseCommand,
) -> anyhow::Result<()> {
  let version = cmd.version.clone().unwrap_or("local".to_string());
  let version_safe = name::encode(&version)?;

  let installs_dir = config.apvm_installs_dir.join("local");
  let target_dir = installs_dir.join(&version_safe);
  let link_src = fs::read_link(&target_dir)?;
  let use_dir = get_use_dir(&config, &cmd)?;

  let target_static = use_dir.join("static");
  let target_bin = use_dir.join("bin");

  if !fs::exists(&target_dir)? {
    return Err(anyhow::anyhow!("Version not installed"));
  }

  if fs::exists(&use_dir)? {
    fs::remove_dir_all(&use_dir)?;
  }

  fs::create_dir_all(&use_dir)?;
  fs::create_dir_all(&target_bin)?;

  link::hard_link_or_copy(&config.exe_path, &target_bin.join("atlaspack"))?;
  link::soft_link(&target_dir, &target_static)?;

  println!("Using: {} ({})", version, link_src.try_to_string()?);
  Ok(())
}

fn get_use_dir(
  config: &Config,
  cmd: &UseCommand,
) -> anyhow::Result<PathBuf> {
  Ok(match cmd.global {
    true => config.apvm_global_dir.clone(),
    false => match config.apvm_active_dir.clone() {
      Some(apvm_active_dir) => apvm_active_dir,
      None => return Err(anyhow::anyhow!("No session, run apvm env first")),
    },
  })
}
