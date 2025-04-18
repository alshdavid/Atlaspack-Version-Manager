// Hidden utilities to access dynamically set values
use clap::Parser;
use clap::Subcommand;

use crate::config::Config;
use crate::platform::active::ActivePackage;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec_blocking;
use crate::platform::path_ext::PathExt;
use crate::platform::runtime::resolve_runtime;

#[derive(Debug, Subcommand, Clone)]
pub enum InfoCommandType {
  LinkPath,
  RealPath,
  Resolve { specifier: String },
  Kind,
}

#[derive(Debug, Parser)]
pub struct ResolveCommand {
  #[clap(subcommand)]
  pub query: InfoCommandType,
}

#[rustfmt::skip]
pub async fn main(config: Config, cmd: ResolveCommand) -> anyhow::Result<()> {
  
  match cmd.query {
    InfoCommandType::RealPath => {
      if let Some(active) = ActivePackage::new(&config)? {
        print!("{}", active.real_path.try_to_string()?);
      };
      Ok(())
    },
    InfoCommandType::LinkPath => {
      if let Some(active) = ActivePackage::new(&config)? {
        print!("{}", active.link_path.try_to_string()?);
      };
      Ok(())
    },
    InfoCommandType::Kind => {
      if let Some(active) = ActivePackage::new(&config)? {
        print!("{}", active.kind);
      };
      Ok(())
    },
    InfoCommandType::Resolve{specifier}=> {
      let runtime = resolve_runtime("node")?;

      exec_blocking([&runtime.try_to_string()?, "-e", &format!("console.log(require.resolve('{}'))", specifier)], ExecOptions {
        cwd: Some(config.apvm_active_dir.join("static")),
        silent: false,
        env: None,
      })?;

      Ok(())
    },
  }
}
