use std::fs;
use std::time::SystemTime;

use tar::Archive;
use xz::read::XzDecoder;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::constants as c;
use crate::platform::name;
use crate::platform::temp_dir::TempDir;

pub async fn install_from_super(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let start_time = SystemTime::now();

  let version_safe = name::encode(&cmd.version)?;
  let specifier = cmd.version;

  let target_temp = TempDir::new(&config.apvm_dir_temp.join(format!("{}.temp", version_safe)));

  let target = config.apvm_installs_dir.join("super").join(&version_safe);

  if target.exists() && (cmd.force || specifier == "main") {
    println!("Removing existing");
    fs::remove_dir_all(&target)?;
  } else if !cmd.force && target.exists() {
    return Err(anyhow::anyhow!("Already installed",));
  }

  let url = format!(
    "https://github.com/alshdavid-forks/atlaspack/releases/download/{}/{}",
    &specifier,
    c::TARBALL,
  );

  println!("Downloading: {}", url);

  let response = reqwest::get(url).await?;

  if response.status() == 404 {
    return Err(anyhow::anyhow!("Version '{}' not found", &specifier));
  }

  let bytes = response.bytes().await?.to_vec();

  println!("Extracting");
  let tar = XzDecoder::new(bytes.as_slice());
  let mut archive = Archive::new(tar);

  archive.unpack(&target_temp)?;

  let Some(Ok(inner_temp)) = fs::read_dir(&target_temp)?.next() else {
    return Err(anyhow::anyhow!("Unable to find inner package"));
  };

  fs::rename(inner_temp.path(), &target)?;

  println!(
    "✅ Installed in {:.2?} ({})",
    start_time.elapsed()?,
    specifier
  );

  Ok(())
}
