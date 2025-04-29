use std::fs;

use clap::Parser;

use crate::config::Config;
use crate::platform::colors::*;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser)]
pub struct ListCommand {}

pub fn main(
  config: Config,
  _cmd: ListCommand,
) -> anyhow::Result<()> {
  for entry in fs::read_dir(&config.paths.versions_npm)? {
    let entry = entry?.path();
    let package = PackageDescriptor::parse_from_dir(&config, &entry)?;

    print_name(
      &package.version,
      config
        .active_version
        .as_ref()
        .is_some_and(|v| v.package == package),
      "",
    );
  }

  for entry in fs::read_dir(&config.paths.versions_local)? {
    let entry = entry?.path();
    let package = PackageDescriptor::parse_from_dir(&config, &entry)?;

    print_name(
      &package.version.to_string(),
      config
        .active_version
        .as_ref()
        .is_some_and(|v| v.package == package),
      "(local) ",
    );
  }

  for entry in fs::read_dir(&config.paths.versions_git)? {
    let entry = entry?.path();
    let package = PackageDescriptor::parse_from_dir(&config, &entry)?;

    print_name(
      &package.version,
      config
        .active_version
        .as_ref()
        .is_some_and(|v| v.package == package),
      "(git) ",
    );
  }

  Ok(())
}

fn print_name(
  name: &str,
  active: bool,
  prefix: &str,
) {
  if active {
    println!("{color_blue}{style_bold}* {prefix}{name}{color_reset}{style_reset}");
  } else {
    println!("* {prefix}{name}");
  }
}
