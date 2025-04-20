use clap::Parser;
use clap::ValueEnum;
use rand::Rng;
use rand::distr::Alphanumeric;

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
  pub shell: Option<Shell>,
}

pub async fn main(
  config: Config,
  cmd: EnvCommand,
) -> anyhow::Result<()> {
  let id = match std::env::var("APVM_SESSION") {
    Ok(id) => id,
    Err(_) => rand::rng()
      .sample_iter(&Alphanumeric)
      .take(15)
      .map(char::from)
      .collect::<String>(),
  };

  let current_shell = std::env::var("SHELL");

  match cmd.shell {
    Some(Shell::Bash) => print!("{}", bash(config, id)?),
    Some(Shell::Zsh) => print!("{}", bash(config, id)?),
    None => match current_shell.as_ref().map(|v| v.as_str()) {
      Ok(shell) => {
        if shell.contains("/bash") || shell.contains("/zsh") {
          print!("{}", bash(config, id)?)
        } else {
          return Err(anyhow::anyhow!(
            "Unable to determine shell. Please specify using -s \"bash\""
          ));
        }
      }
      _ => {
        return Err(anyhow::anyhow!(
          "Unable to determine shell. Please specify using -s \"bash\""
        ));
      }
    },
  };

  Ok(())
}

fn bash(
  config: Config,
  id: String,
) -> anyhow::Result<String> {
  Ok(format!(
    r#"
export APVM_SESSION={};
export APVM_DIR={};
export PATH={}/sessions/{}/bin:$PATH;
trap '{} unload' EXIT;
"#,
    id,
    config.apvm_dir.try_to_string()?,
    config.apvm_dir.try_to_string()?,
    id,
    config.exe_path.try_to_string()?,
  ))
}
