use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser, Clone)]
pub struct NpmLinkCommand {
  /// Target version to link
  pub version: Option<String>,

  /// Link with support for @atlaspack/* packages
  #[arg(short = 'i', long = "install")]
  pub legacy: bool,
}

pub async fn npm_link(
  _config: Config,
  _cmd: NpmLinkCommand,
) -> anyhow::Result<()> {
  todo!()
}
