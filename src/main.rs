// #![deny(unused_crate_dependencies)]

mod cmd;
mod config;
mod env;
mod platform;
mod log;

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;
use env::Env;

#[derive(Debug, Subcommand)]
pub enum ApvmCommandType {
  /// Set the default version of Atlaspack
  Default(cmd::default::DefaultCommand),
  /// Install a version of Atlaspack
  Install(cmd::install::InstallCommand),
  /// Helpers to work with node_modules
  Npm(cmd::npm::NpmCommand),
  /// List installed versions of Atlaspack
  List(cmd::list::ListCommand),
  /// Reinstall a previously installed version of Atlaspack
  Reinstall(cmd::install::InstallCommand),
  /// Uninstall a previously installed version of Atlaspack
  Uninstall(cmd::uninstall::UninstallCommand),
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
  #[arg(env = "RUST_LOG")]
  pub _rust_log: Option<log::Level>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init();
  let env = Env::parse()?;
  let config = config::Config::new(&env)?;

  // If the executable is called "atlaspack" then only proxy
  if &config.exe_stem == "atlaspack" {
    return cmd::proxy::main(config).await;
  }

  // Calling "apvm atlaspack" will proxy to the active Atlaspack version
  if let Some("atlaspack") = config.argv.first().map(|v| v.as_str()) {
    let mut config = config;
    config.argv.remove(0);
    return cmd::proxy::main(config).await;
  }

  // APVM Commands
  let args = ApvmCommand::parse();

  match args.command {
    ApvmCommandType::Install(cmd) => cmd::install::main(config, cmd).await,
    ApvmCommandType::Reinstall(cmd) => cmd::reinstall::main(config, cmd).await,
    ApvmCommandType::List(cmd) => cmd::list::main(config, cmd).await,
    ApvmCommandType::Uninstall(cmd) => cmd::uninstall::main(config, cmd).await,
    ApvmCommandType::Version => cmd::version::main(config).await,
    ApvmCommandType::Debug(cmd) => cmd::debug::main(config, cmd).await,
    ApvmCommandType::Default(cmd) => cmd::default::main(config, cmd).await,
    ApvmCommandType::Npm(cmd) => cmd::npm::main(config, cmd).await,
    ApvmCommandType::Atlaspack => unreachable!(),
  }
}
