mod cmd;
mod config;
mod platform;
mod env;

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use env::Env;

#[derive(Debug, Subcommand)]
pub enum ApvmCommandType {
  /// Print environment variables for apvm
  Env(cmd::env::EnvCommand),
  /// Install a version of Atlaspack
  Install(cmd::install::InstallCommand),
  /// Links the active version of Atlaspack into the current directory
  Link(cmd::link::LinkCommand),
  /// List installed versions of Atlaspack
  List(cmd::list::ListCommand),
  /// Run command with an installed versions of Atlaspack
  Run(cmd::run::RunCommand),
  /// Uninstall a previously installed version of Atlaspack
  Uninstall(cmd::uninstall::UninstallCommand),
  /// Unload the current session
  Unload,
  /// Use an installed version of Atlaspack
  Use(cmd::r#use::UseCommand),
  /// Version information
  Version,
}

#[derive(Parser, Debug)]
pub struct ApvmCommand {
  #[clap(subcommand)]
  pub command: ApvmCommandType,
  #[arg(env = "APVM_DIR")]
  pub apvm_dir: Option<PathBuf>,
  #[arg( env = "APVM_LOCAL")]
  pub apvm_local: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let env = Env::parse()?;
  let config = config::Config::new(&env)?;

  // dbg!(&config);

  // Validate session is up
  if config.argv.get(0).is_some_and(|v| v != "env" && env.apvm_session.is_none()) {
    return Err(anyhow::anyhow!("Run 'apvm env' first"))
  }

  // Commands to proxy
  match config.argv.get(0).map(|v| v.as_str()) {
    Some("build") => return cmd::build::main(config).await,
    Some("watch") => return cmd::build::main(config).await,
    _ => {}
  }

  // APVM Commands
  let args = ApvmCommand::parse();

  
  match args.command {
    ApvmCommandType::Install(cmd) => cmd::install::main(config, cmd).await,
    ApvmCommandType::Use(cmd) => cmd::r#use::main(config, cmd).await,
    ApvmCommandType::List(cmd) => cmd::list::main(config, cmd).await,
    ApvmCommandType::Link(cmd) => cmd::link::main(config, cmd).await,
    ApvmCommandType::Run(cmd) => cmd::run::main(config, cmd).await,
    ApvmCommandType::Uninstall(cmd) => cmd::uninstall::main(config, cmd).await,
    ApvmCommandType::Unload => cmd::unload::main(config).await,
    ApvmCommandType::Env(cmd) => cmd::env::main(config, cmd).await,
    ApvmCommandType::Version => cmd::version::main(config).await,
  }
}
