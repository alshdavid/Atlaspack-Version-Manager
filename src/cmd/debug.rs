// Hidden utilities to access dynamically set values
use clap::Parser;
use clap::Subcommand;

use crate::config::Config;
// use crate::platform::active::ActivePackage;

#[derive(Debug, Subcommand, Clone)]
pub enum DebugCommandType {
  LinkPath,
  RealPath,
  Resolve { specifier: String },
}

#[derive(Debug, Parser)]
pub struct DebugCommand {
  #[clap(subcommand)]
  pub query: Option<DebugCommandType>,
}

#[rustfmt::skip]
pub fn main(config: Config, cmd: DebugCommand) -> anyhow::Result<()> {
  
  match cmd.query {
    None => {
      dbg!(&config);
    },
    Some(DebugCommandType::RealPath ) => {
      // let active = ActivePackage::try_active_or_global(&config)?;
      // print!("{}", active.static_path_real.try_to_string()?);
    },
    Some(DebugCommandType::LinkPath ) => {
      // let active = ActivePackage::try_active_or_global(&config)?;
      // print!("{}", active.static_path.try_to_string()?);
    },
    Some(DebugCommandType::Resolve{specifier: _}) => {
      // let runtime = resolve_runtime("node")?;
      // let active = ActivePackage::try_active_or_global(&config)?;
      // exec_blocking([&runtime.try_to_string()?, "-e", &format!("console.log(require.resolve('{specifier}'))")], ExecOptions {
      //   cwd: Some(active.static_path_real),
      //   silent: false,
      //   env: None,
      // })
    },
  }

  Ok(())
}
