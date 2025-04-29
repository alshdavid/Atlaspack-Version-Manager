use std::fs;

use flate2::read::GzDecoder;
use tar::Archive;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec_blocking;
use crate::platform::package::PackageDescriptor;
use crate::platform::temp_dir::TempDir;

pub fn install_from_git(
  config: Config,
  cmd: InstallCommand,
  package: PackageDescriptor,
) -> anyhow::Result<()> {
  let target_temp = TempDir::new(&config.paths.temp)?;
  let target = config.paths.versions_git.join(&package.version_encoded);

  if target.exists() && cmd.force {
    println!("Removing existing");
    fs::remove_dir_all(&target)?;
  } else if !cmd.force && target.exists() {
    println!("✅ Already installed");
    return Ok(());
  }

  let url = format!(
    "https://github.com/atlassian-labs/atlaspack/archive/{}.tar.gz",
    &package.version
  );

  println!("Fetching {}", &url);
  let response = reqwest::blocking::get(&url)?;
  if response.status() == 404 {
    return Err(anyhow::anyhow!("Version '{}' not found", &package.version));
  }

  println!("Downloading");
  let bytes = response.bytes()?.to_vec();

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
    return Ok(());
  }

  println!("Initializing");
  // Atlaspack needs a .git folder or the build will fail
  fs::create_dir_all(inner_temp.path().join(".git"))?;

  println!("Installing (yarn)");
  exec_blocking(["yarn", "install"], command_options.clone())?;

  println!("Building (Native)");
  exec_blocking(["yarn", "build-native-release"], command_options.clone())?;

  println!("Building (Flow)");
  exec_blocking(["yarn", "build"], command_options.clone())?;

  println!("Building (TypeScript)");
  exec_blocking(["yarn", "build-ts"], command_options.clone())?;

  fs::rename(inner_temp.path(), &target)?;

  Ok(())
}
