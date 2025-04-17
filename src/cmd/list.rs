use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::active::ActivePackage;
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
  let active = ActivePackage::new(&config)?;

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
