use clap::Parser;

use super::r#use::UseCommand;
use crate::config::Config;
use crate::platform::origin::InstallOrigin;

#[derive(Debug, Parser)]
pub struct GlobalCommand {
  /// Target version to use
  pub version: Option<String>,

  #[arg(short = 'o', long = "origin", default_value = "super")]
  pub origin: Option<InstallOrigin>,
}

pub async fn main(
  config: Config,
  cmd: GlobalCommand,
) -> anyhow::Result<()> {
  super::r#use::main(
    config,
    UseCommand {
      version: cmd.version,
      global: true,
      origin: cmd.origin,
    },
  )
  .await
}
