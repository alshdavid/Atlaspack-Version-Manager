use std::fs;

use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;
use tokio::process::Command;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct InstallCommand {
  /// Target version to install
  pub version: String,

  /// Replace an existing version if already installed
  #[arg(short = 'f', long = "force")]
  pub force: bool,
}

pub async fn main(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  install_from_git(config, cmd).await
}

async fn install_from_git(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let version_safe = urlencoding::encode(&cmd.version).to_string();

  let target_temp = config
    .apvm_installs_dir
    .join(format!("{}.temp", version_safe));

  let target = config.apvm_installs_dir.join(&version_safe);

  if cmd.force || version_safe == "main" && target.exists() {
    fs::remove_dir_all(&target)?;
  } else if !cmd.force && target.exists() {
    return Err(anyhow::anyhow!("Already installed",));
  }

  println!(
    "Fetching https://github.com/atlassian-labs/atlaspack/archive/{}.tar.gz",
    &cmd.version,
  );

  let response = reqwest::get(format!(
    "https://github.com/atlassian-labs/atlaspack/archive/{}.tar.gz",
    &cmd.version,
  ))
  .await?;

  if response.status() == 404 {
    return Err(anyhow::anyhow!("Version '{}' not found", &cmd.version));
  }

  println!("Downloading...");
  let bytes = response.bytes().await?.to_vec();

  println!("Extracting...");
  let tar = GzDecoder::new(bytes.as_slice());
  let mut archive = Archive::new(tar);

  archive.unpack(&target_temp)?;

  let Some(Ok(inner)) = fs::read_dir(&target_temp)?.next() else {
    return Err(anyhow::anyhow!("Unable to find inner package"));
  };

  fs::rename(inner.path(), &target)?;
  fs::remove_dir_all(&target_temp)?;

  println!("Installing...");
  fs::create_dir(target.join(".git"))?;
  Command::new("/usr/bin/env")
    .args(["yarn", "install"])
    .current_dir(&target)
    .env("HUSKY", "0")
    .spawn()?
    .wait()
    .await?;

  println!("Building (Native)...");
  Command::new("/usr/bin/env")
    .args(["yarn", "build-native-release"])
    .current_dir(&target)
    .spawn()?
    .wait()
    .await?;

  println!("Building (Flow)...");
  Command::new("/usr/bin/env")
    .args(["yarn", "build"])
    .current_dir(&target)
    .spawn()?
    .wait()
    .await?;

  println!("Building (TypeScript)...");
  Command::new("/usr/bin/env")
    .args(["yarn", "build-ts"])
    .current_dir(&target)
    .spawn()?
    .wait()
    .await?;

  println!("Installed");

  Ok(())
}
