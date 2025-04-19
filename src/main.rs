#![deny(unused_crate_dependencies)]

mod cmd;
mod config;
mod env;
mod platform;

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
  /// Run command with specified version of atlaspack
  Atlaspack
  ,
  #[clap(hide = true)]
  Info(cmd::info::ResolveCommand),
}

#[derive(Parser, Debug)]
pub struct ApvmCommand {
  #[clap(subcommand)]
  pub command: ApvmCommandType,
  #[arg(env = "APVM_DIR")]
  pub apvm_dir: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let env = Env::parse()?;
  let config = config::Config::new(&env)?;

  // If the executable is called atlaspack then only proxy
  if &config.exe_stem == "atlaspack" {
    return cmd::proxy::main(config).await;
  }

  // Validate session is up
  if config
    .argv
    .first()
    .is_some_and(|v| v != "env" && env.apvm_session.is_none())
  {
    return Err(anyhow::anyhow!("Run 'apvm env' first"));
  }

  println!("{:?}", config.argv.first());

  // Commands to proxy
  match config.argv.first().map(|v| v.as_str()) {
    Some("atlaspack") => {
      let mut config = config;
      config.argv.remove(0);
      return cmd::proxy::main(config).await;
    }
    Some("build") => return cmd::proxy::main(config).await,
    Some("watch") => return cmd::proxy::main(config).await,
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
    ApvmCommandType::Info(cmd) => cmd::info::main(config, cmd).await,
    ApvmCommandType::Atlaspack => panic!(),
  }
}
