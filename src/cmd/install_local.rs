use std::fs;
use std::path::PathBuf;

use super::install::InstallCommand;
use crate::config::Config;
use crate::platform::link;
use crate::platform::name;
use crate::platform::path_ext::*;

pub async fn install_from_local(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  let Some(version) = cmd.version else {
    return Err(anyhow::anyhow!("Version not specified"));
  };
  let original_path = PathBuf::from(version);

  let alias = match cmd.alias {
    Some(alias) => alias,
    None => "local".to_string(),
  };
  let alias_encoded = name::encode(&alias)?;
  let link_path = config.apvm_installs_dir.join("local").join(&alias_encoded);

  println!("Linking");
  if cmd.force && fs::exists(&link_path)? {
    fs::remove_file(&link_path)?;
  } else if fs::exists(&link_path)? {
    println!(
      "✅ Already Installed local://{}",
      original_path.try_to_string()?
    );
    return Ok(());
  }

  if !fs::exists(&original_path)? {
    return Err(anyhow::anyhow!(
      "Does not exist: {}",
      original_path.try_to_string()?
    ));
  }

  link::soft_link(&original_path, &link_path)?;
  println!("✅ Installed local://{}", original_path.try_to_string()?);

  Ok(())
}
