use std::fs;

use flate2::read::GzDecoder;
use tar::Archive;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::constants as c;
use crate::platform::package::PackageDescriptor;
use crate::platform::temp_dir::TempDir;

pub async fn install_from_npm(
  config: Config,
  _cmd: InstallCommand,
  package: PackageDescriptor,
) -> anyhow::Result<()> {
  let target_temp = TempDir::new(&config.paths.temp.join(&package.version_encoded));
  let target = config.paths.versions_npm.join(&package.version_encoded);

  let url = format!(
    "https://github.com/alshdavid-forks/atlaspack/releases/download/{}/{}",
    package.version,
    c::TARBALL
  );

  println!("Fetching");
  let response = reqwest::get(&url).await?;
  if response.status() != 200 {
    return Err(anyhow::anyhow!("Unable to fetch version"));
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

  fs::rename(inner_temp.path(), &target)?;

  Ok(())
}
