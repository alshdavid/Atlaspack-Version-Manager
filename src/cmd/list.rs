use std::fs;
use std::path::PathBuf;

use clap::Parser;

use crate::config::Config;
use crate::platform::colors::*;
use crate::platform::name;
use crate::platform::origin::InstallOrigin;
use crate::platform::path_ext::*;

#[derive(Debug, Parser)]
pub struct ListCommand {}

pub async fn main(
  config: Config,
  _cmd: ListCommand,
) -> anyhow::Result<()> {
  let active = Active::new(&config)?;

  for entry in fs::read_dir(config.apvm_installs_dir.join("local"))? {
    let entry = entry?;
    let file_name = name::decode(entry.file_name().try_to_string()?)?;
    let link_src = fs::read_link(entry.path())?;
    print_name(
      &file_name,
      active
        .as_ref()
        .is_some_and(|a| a.kind == InstallOrigin::Local && a.name == file_name),
      &format!("({})", link_src.try_to_string()?),
    );
  }

  for entry in fs::read_dir(config.apvm_installs_dir.join("super"))? {
    let file_name = name::decode(entry?.file_name().try_to_string()?)?;
    print_name(
      &file_name,
      active
        .as_ref()
        .is_some_and(|a| a.kind == InstallOrigin::Super && a.name == file_name),
      "",
    );
  }

  for entry in fs::read_dir(config.apvm_installs_dir.join("git"))? {
    let file_name = name::decode(entry?.file_name().try_to_string()?)?;
    print_name(
      &file_name,
      active
        .as_ref()
        .is_some_and(|a| a.kind == InstallOrigin::Git && a.name == file_name),
      "(git)",
    );
  }

  Ok(())
}

fn print_name(
  name: &str,
  active: bool,
  suffix: &str,
) {
  if active {
    println!(
      "{}{}* {} {}{}{}",
      color_blue, style_bold, name, suffix, color_reset, style_reset
    );
  } else {
    println!("* {} {}", name, suffix);
  }
}

#[allow(unused)]
struct Active {
  kind: InstallOrigin,
  name_encoded: String,
  name: String,
  path: PathBuf,
}

impl Active {
  fn new(config: &Config) -> anyhow::Result<Option<Self>> {
    let target_path = config.apvm_active_dir.join("static");
    if !fs::exists(&target_path)? {
      return Ok(None);
    }
    let link = fs::read_link(config.apvm_active_dir.join("static"))?;
    let link_name_encoded = link.try_file_name()?;
    let link_name = name::decode(&link_name_encoded)?;
    let link_kind = link.try_parent()?.try_file_name()?;
    Ok(Some(Self {
      kind: InstallOrigin::try_from(link_kind)?,
      name_encoded: link_name_encoded,
      name: link_name,
      path: link,
    }))
  }
}
