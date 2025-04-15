use std::fs;

use clap::Parser;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::config::Config;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec;
use crate::platform::name;

#[derive(Debug, Parser)]
pub struct InstallCommand {
  /// Target version to install
  pub version: String,

  /// Replace an existing version if already installed
  #[arg(short = 'f', long = "force")]
  pub force: bool,

  /// Forward stdout/stderr for the underlying commands
  #[arg(short = 'v', long = "verbose")]
  pub verbose: bool,
}

pub async fn main(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  if cmd.version == "local" {
    return Err(anyhow::anyhow!(
      "Cannot install local version\n Run:\n\tapvm use local"
    ));
  }

  // Installs and builds Atlaspack from git
  if cmd.version.starts_with("git:") {
    return install_from_git(config, cmd).await;
  }

  // [TODO] Add super package

  Err(anyhow::anyhow!("No handler for specifier"))
}

async fn install_from_git(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let version = cmd.version;
  let version_safe = name::encode(&version)?;
  let branch = version.replacen("git:", "", 1);

  let target_temp = config
    .apvm_installs_dir
    .join(format!("{}.temp", version_safe));

  let target = config.apvm_installs_dir.join(&version_safe);

  if cmd.force || version == "main" && target.exists() {
    fs::remove_dir_all(&target)?;
  } else if !cmd.force && target.exists() {
    return Err(anyhow::anyhow!("Already installed",));
  }

  println!(
    "🚀 Fetching https://github.com/atlassian-labs/atlaspack/archive/{}.tar.gz",
    &branch,
  );

  let response = reqwest::get(format!(
    "https://github.com/atlassian-labs/atlaspack/archive/{}.tar.gz",
    &branch,
  ))
  .await?;

  if response.status() == 404 {
    return Err(anyhow::anyhow!("Version '{}' not found", &version));
  }

  println!("📩 Downloading...");
  let bytes = response.bytes().await?.to_vec();

  println!("📤 Extracting...");
  let tar = GzDecoder::new(bytes.as_slice());
  let mut archive = Archive::new(tar);

  archive.unpack(&target_temp)?;

  let Some(Ok(inner)) = fs::read_dir(&target_temp)?.next() else {
    return Err(anyhow::anyhow!("Unable to find inner package"));
  };

  fs::rename(inner.path(), &target)?;
  fs::remove_dir_all(&target_temp)?;

  let command_options = ExecOptions {
    cwd: Some(target),
    silent: !cmd.verbose,
    ..Default::default()
  };

  println!("🤖 Initializing...");
  exec(["git", "init"], command_options.clone()).await?;
  exec(["git", "add", "."], command_options.clone()).await?;
  exec(
    ["git", "commit", "-m", "Initial Commit"],
    command_options.clone(),
  )
  .await?;

  println!("🧶 Installing... (yarn)");
  exec(["yarn", "install"], command_options.clone()).await?;

  println!("🔨 Building (Native)...");
  exec(["yarn", "build-native-release"], command_options.clone()).await?;

  println!("🔨 Building (Flow)...");
  exec(["yarn", "build"], command_options.clone()).await?;

  println!("🔨 Building (TypeScript)...");
  exec(["yarn", "build-ts"], command_options.clone()).await?;

  println!("✅ Installed");

  Ok(())
}
