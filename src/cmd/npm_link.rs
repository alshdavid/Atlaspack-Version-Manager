use clap::Parser;

use super::npm_link_npm::npm_link_npm;
use crate::cmd::install::InstallCommand;
use crate::context::Context;
use crate::platform::origin::VersionTarget;
use crate::platform::package::PackageDescriptor;

#[derive(Debug, Parser, Clone)]
pub struct NpmLinkCommand {
  /// Target version to link
  pub version: Option<String>,

  /// Install if version doesn't exists
  #[arg(short = 'i', long = "install")]
  pub install: bool,
}

pub fn npm_link(
  ctx: Context,
  cmd: NpmLinkCommand,
) -> anyhow::Result<()> {
  let version = VersionTarget::resolve(&ctx.apvmrc, &cmd.version)?;
  let package = PackageDescriptor::parse(&ctx.paths, &version)?;

  if !package.exists()? && cmd.install {
    super::install::main(
      ctx.clone(),
      InstallCommand {
        version: cmd.version.clone(),
        force: true,
        skip_build: false,
      },
    )?;
  } else if !package.exists()? {
    return Err(anyhow::anyhow!("Version not installed"));
  };

  println!("Linking {}", package.version_target);

  match version {
    VersionTarget::Npm(_) => npm_link_npm(ctx, cmd, package)?,
    VersionTarget::Git(_) => npm_link_git(ctx, cmd, package)?,
    VersionTarget::Local(_) => npm_link_local(ctx, cmd, package)?,
  }

  println!("✅ Link completed");
  Ok(())
}

fn npm_link_git(
  _ctx: Context,
  _cmd: NpmLinkCommand,
  _package: PackageDescriptor,
) -> anyhow::Result<()> {
  println!("TODO");
  Ok(())
}

fn npm_link_local(
  _ctx: Context,
  _cmd: NpmLinkCommand,
  _package: PackageDescriptor,
) -> anyhow::Result<()> {
  println!("TODO");
  Ok(())
}
