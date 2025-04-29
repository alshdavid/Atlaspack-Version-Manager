use clap::Parser;
use clap::Subcommand;

use super::npm_link::NpmLinkCommand;
use crate::config::Config;

#[derive(Debug, Subcommand, Clone)]
pub enum NpmCommandType {
  /// Link a specified version of Atlaspack into node_modules
  Link(NpmLinkCommand),
  /// Scans node_modules recursively for all instances of Atlaspack
  Scan,
  /// Traverse node_modules recursively and ensure only one version of Atlaspack is installed
  Dedupe,
}

#[derive(Debug, Parser)]
pub struct NpmCommand {
  #[clap(subcommand)]
  pub command: NpmCommandType,
}

pub fn main(
  config: Config,
  cmd: NpmCommand,
) -> anyhow::Result<()> {
  match cmd.command {
    NpmCommandType::Link(cmd) => super::npm_link::npm_link(config, cmd),
    NpmCommandType::Scan => todo!(),
    NpmCommandType::Dedupe => todo!(),
  }
}
