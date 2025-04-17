use std::fs;

use flate2::read::GzDecoder;
use tar::Archive;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec_blocking;
use crate::platform::name;
use crate::platform::temp_dir::TempDir;

pub async fn install_from_git(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let version_safe = name::encode(&cmd.version)?;
  let branch = cmd.version;

  let target_temp = TempDir::new(&config.apvm_dir_temp.join(format!("{}.temp", version_safe)));

  let target = config.apvm_installs_dir.join("git").join(&version_safe);

  if target.exists() && (cmd.force || branch == "main") {
    println!("Removing existing");
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
    return Err(anyhow::anyhow!("Version '{}' not found", &branch));
  }

  println!("Downloading");
  let bytes = response.bytes().await?.to_vec();

  println!("Extracting");
  let tar = GzDecoder::new(bytes.as_slice());
  let mut archive = Archive::new(tar);

  archive.unpack(&target_temp)?;

  let Some(Ok(inner_temp)) = fs::read_dir(&target_temp)?.next() else {
    return Err(anyhow::anyhow!("Unable to find inner package"));
  };

  let command_options = ExecOptions {
    cwd: Some(inner_temp.path()),
    silent: !cmd.verbose,
    ..Default::default()
  };

  if cmd.skip_build {
    fs::rename(inner_temp.path(), &target)?;
    println!("Skipping build steps");
    println!("✅ Installed git://atlassian-labs/atlaspack/{}", branch);
    return Ok(());
  }

  println!("Initializing");
  exec_blocking(["git", "init"], command_options.clone())?;
  exec_blocking(["git", "add", "."], command_options.clone())?;
  exec_blocking(
    ["git", "commit", "-m", "Initial Commit"],
    command_options.clone(),
  )?;

  println!("Installing (yarn)");
  exec_blocking(["yarn", "install"], command_options.clone())?;

  println!("Building (Native)");
  exec_blocking(["yarn", "build-native-release"], command_options.clone())?;

  println!("Building (Flow)");
  exec_blocking(["yarn", "build"], command_options.clone())?;

  println!("Building (TypeScript)");
  exec_blocking(["yarn", "build-ts"], command_options.clone())?;

  fs::rename(inner_temp.path(), &target)?;

  println!("✅ Installed git://atlassian-labs/atlaspack/{}", branch);

  Ok(())
}
