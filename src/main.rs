mod cmd;
mod config;
mod constants;

use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum ApvmCommandType {
  /// Install a version of Atlaspack
  Install(cmd::install::InstallCommand),
  /// Use an installed version of Atlaspack
  Use(cmd::r#use::UseCommand),
  /// List installed versions of Atlaspack
  List(cmd::list::ListCommand),
  /// Run command with an installed versions of Atlaspack
  Run(cmd::run::RunCommand),
  /// Uninstall a previously installed version of Atlaspack
  Uninstall(cmd::uninstall::UninstallCommand),
  /// Command to env
  Env(cmd::env::EnvCommand),
  /// Version information
  Version(cmd::version::VersionCommand),
}

#[derive(Parser, Debug)]
pub struct ApvmCommand {
  #[clap(subcommand)]
  pub command: ApvmCommandType,
  #[arg(long = "apvm-dir", env = "APVM_DIR")]
  pub apvm_dir: Option<PathBuf>,
  #[arg(long = "apvm-local", env = "APVM_LOCAL")]
  pub apvm_local: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = ApvmCommand::parse();
  let config = config::Config::new(&args)?;

  match args.command {
    ApvmCommandType::Install(cmd) => cmd::install::main(config, cmd).await,
    ApvmCommandType::Use(cmd) => cmd::r#use::main(config, cmd).await,
    ApvmCommandType::List(cmd) => cmd::list::main(config, cmd).await,
    ApvmCommandType::Run(cmd) => cmd::run::main(config, cmd).await,
    ApvmCommandType::Uninstall(cmd) => cmd::uninstall::main(config, cmd).await,
    ApvmCommandType::Env(cmd) => cmd::env::main(config, cmd).await,
    ApvmCommandType::Version(cmd) => cmd::version::main(config, cmd).await,
  }
}
