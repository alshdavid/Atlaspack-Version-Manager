use super::install::InstallCommand;
use crate::config::Config;

pub fn main(
  config: Config,
  cmd: InstallCommand,
) -> anyhow::Result<()> {
  super::install::main(config, InstallCommand { force: true, ..cmd })
}
