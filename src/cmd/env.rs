use std::env;

use clap::Parser;
use clap::ValueEnum;

use crate::config::Config;

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
    Shell::Bash => print!("{}", bash(config)),
    Shell::Zsh => print!("{}", bash(config)),
  };

  Ok(())
}

fn bash(config: Config) -> String {
  let exe_path = env::current_exe().unwrap();
  format!(
    r#"
export PATH="{}/bin:$PATH";
export APVM_SESSION={};
trap '{} unload' EXIT;
"#,
    config.apvm_active_dir.to_str().unwrap(),
    config.id,
    exe_path.to_str().unwrap(),
  )
}
