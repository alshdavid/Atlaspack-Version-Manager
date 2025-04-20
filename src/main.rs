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
  /// Set the global version of Atlaspack to use
  Global(cmd::global::GlobalCommand),
  /// Install a version of Atlaspack
  Install(cmd::install::InstallCommand),
  /// Link Atlaspack into node_modules
  Link(cmd::link::LinkCommand),
  /// List installed versions of Atlaspack
  List(cmd::list::ListCommand),
  /// Uninstall a previously installed version of Atlaspack
  Uninstall(cmd::uninstall::UninstallCommand),
  /// Use an installed version of Atlaspack
  Use(cmd::r#use::UseCommand),
  /// Version information
  Version,
  /// Run command with specified version of atlaspack
  Atlaspack,
  #[clap(hide = true)]
  Debug(cmd::debug::DebugCommand),
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

  // Commands to proxy
  if let Some("atlaspack") = config.argv.first().map(|v| v.as_str()) {
    let mut config = config;
    config.argv.remove(0);
    return cmd::proxy::main(config).await;
  }

  // APVM Commands
  let args = ApvmCommand::parse();

  match args.command {
    ApvmCommandType::Install(cmd) => cmd::install::main(config, cmd).await,
    ApvmCommandType::Global(cmd) => cmd::global::main(config, cmd).await,
    ApvmCommandType::Use(cmd) => cmd::r#use::main(config, cmd).await,
    ApvmCommandType::List(cmd) => cmd::list::main(config, cmd).await,
    ApvmCommandType::Link(cmd) => cmd::link::main(config, cmd).await,
    ApvmCommandType::Uninstall(cmd) => cmd::uninstall::main(config, cmd).await,
    ApvmCommandType::Env(cmd) => cmd::env::main(config, cmd).await,
    ApvmCommandType::Version => cmd::version::main(config).await,
    ApvmCommandType::Debug(cmd) => cmd::debug::main(config, cmd).await,
    ApvmCommandType::Atlaspack => panic!(),
  }
}
