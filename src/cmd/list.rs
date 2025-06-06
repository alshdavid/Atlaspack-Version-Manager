use std::fs;

use clap::Parser;

use crate::context::Context;
use crate::platform::colors::*;
use crate::platform::package::PackageDescriptor;
use crate::platform::path_ext::*;

#[derive(Debug, Parser)]
pub struct ListCommand {}

pub fn main(
  ctx: Context,
  _cmd: ListCommand,
) -> anyhow::Result<()> {
  if let Some(apvmrc) = &ctx.apvmrc {
    let mut project_versions = vec![];

    if let Some(default) = &apvmrc.version_target {
      project_versions.push((default.to_string(), "".to_string()));
    }

    for (alias, version) in &apvmrc.version_target_aliases {
      project_versions.push((version.to_string(), format!("({}) ", alias)));
    }

    if !project_versions.is_empty() {
      println!("{style_underline}Project Versions{style_reset} package.json#atlaspack");
      for (version, alias) in project_versions {
        print_name(&version, false, &alias);
      }
      println!();
    }
  }

  println!("{style_underline}Active Version{style_reset}");
  if let Some(active) = ctx.active_version {
    println!("  Version: {}", active.package.version_target);
    println!("  From:    {:?}", active.active_type);
  } else {
    println!("  <No Active Version>");
  }
  println!();

  println!("{style_underline}Installed Versions{style_reset}");

  let npm_versions = fs::read_dir(&ctx.paths.versions_npm)?.collect::<Vec<_>>();
  if !npm_versions.is_empty() {
    for entry in fs::read_dir(&ctx.paths.versions_npm)? {
      let entry = entry?.path();
      let package = PackageDescriptor::parse_from_dir(&ctx.paths, &entry)?;

      print_name(&package.version, false, "");
    }
  } else {
    println!("  <No Versions Installed>");
  }

  let local_versions = fs::read_dir(&ctx.paths.versions_local)?.collect::<Vec<_>>();
  if !local_versions.is_empty() {
    for entry in local_versions {
      let entry = entry?.path();
      let package = PackageDescriptor::parse_from_dir(&ctx.paths, &entry)?;

      print_name(
        &package.path_real()?.try_to_string()?,
        false,
        &format!("({}) ", package.version),
      );
    }
  }

  let git_versions = fs::read_dir(&ctx.paths.versions_git)?.collect::<Vec<_>>();
  if !git_versions.is_empty() {
    for entry in git_versions {
      let entry = entry?.path();
      let package = PackageDescriptor::parse_from_dir(&ctx.paths, &entry)?;

      print_name(&package.version, false, "(git) ");
    }
  }

  Ok(())
}

fn print_name(
  name: &str,
  active: bool,
  prefix: &str,
) {
  if active {
    println!("{color_blue}{style_bold}  {prefix}{name}{color_reset}{style_reset}");
  } else {
    println!("  {prefix}{name}");
  }
}
