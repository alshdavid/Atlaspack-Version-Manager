use clap::Parser;
use clap::ValueEnum;

use crate::config::Config;
use crate::platform::path_ext::PathExt;

#[derive(ValueEnum, Debug, Clone)]
pub enum Shell {
  Bash,
  Zsh,
  // Fish,
  // PowerShell,
  // Cmd,
}

#[derive(Debug, Parser)]
pub struct EnvCommand {
  /// Runtime to use
  #[arg(short = 's', long = "shell")]
  pub shell: Shell,
}

pub async fn main(
  config: Config,
  cmd: EnvCommand,
) -> anyhow::Result<()> {
  match cmd.shell {
    Shell::Bash => print!("{}", bash(config)?),
    Shell::Zsh => print!("{}", bash(config)?),
  };

  Ok(())
}

fn bash(config: Config) -> anyhow::Result<String> {
  Ok(format!(
    r#"
export PATH={}/bin:$PATH;
export APVM_DIR={};
export APVM_SESSION={};
trap '{} unload' EXIT;
"#,
    config.apvm_active_dir.try_to_string()?,
    config.apvm_dir.try_to_string()?,
    config.id,
    config.exe_path.try_to_string()?,
  ))
}
