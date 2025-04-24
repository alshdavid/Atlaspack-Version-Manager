use std::fs;
use std::time::SystemTime;

use flate2::read::GzDecoder;
use tar::Archive;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::constants as c;
use crate::platform::name;
use crate::platform::temp_dir::TempDir;

pub async fn install_from_npm(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let start_time = SystemTime::now();

  let Some(version) = cmd.version else {
    panic!();
  };

  let version_safe = name::encode(&version)?;
  let target_temp = TempDir::new(&config.apvm_dir_temp.join(format!("{version_safe}.temp")));
  let target = config.apvm_installs_dir.join("npm").join(&version_safe);

  let url = format!(
    "https://github.com/alshdavid-forks/atlaspack/releases/download/{}/{}",
    version,
    c::TARBALL
  );
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

  println!(
    "✅ Installed in {:.2?} ({})",
    start_time.elapsed()?,
    version
  );

  Ok(())
}
