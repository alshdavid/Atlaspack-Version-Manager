use clap::Parser;

use crate::config::Config;

#[derive(Debug, Parser)]
pub struct LinkCommand {}

pub async fn main(
  _config: Config,
  _cmd: LinkCommand,
) -> anyhow::Result<()> {
  Ok(())
}
