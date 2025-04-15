use std::collections::HashMap;
use std::fs;

use clap::Parser;
use clap::ValueEnum;

use crate::config::Config;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec_blocking;

#[derive(ValueEnum, Debug, Clone)]
pub enum Runtime {
  Node,
  SystemNode,
  SystemDeno,
}

#[derive(Debug, Parser)]
pub struct RunCommand {
  /// Command to run
  pub command: Vec<String>,

  /// Runtime to use
  #[arg(short = 'r', long = "runtime", default_value = "system-node")]
  pub runtime: Runtime,
}

pub async fn main(
  config: Config,
  cmd: RunCommand,
) -> anyhow::Result<()> {
  if !fs::exists(&config.apvm_active_dir)? {
    return Err(anyhow::anyhow!("No active version installed"));
  }

  let link = fs::read_link(&config.apvm_active_dir)?;

  let runtime = match &cmd.runtime {
    Runtime::Node => link.join("share").join("node"),
    Runtime::SystemNode => which::CanonicalPath::new("node")?.to_path_buf(),
    Runtime::SystemDeno => which::CanonicalPath::new("deno")?.to_path_buf(),
  };

  if !fs::exists(&runtime)? {
    return Err(anyhow::anyhow!("Cannot find runtime executable"));
  }

  let mut args = Vec::<String>::new();
  if let Runtime::SystemDeno = &cmd.runtime {
    args.extend(vec!["--allow-all".to_string()])
  }

  args.push(
    link
      .join("packages")
      .join("core")
      .join("cli")
      .join("lib")
      .join("cli.js")
      .to_str()
      .unwrap()
      .to_string(),
  );
  args.extend(cmd.command);

  let (tx, rx) = tokio::sync::oneshot::channel::<anyhow::Result<()>>();

  // Run on separate thread to allow instant exit on cnt+c
  std::thread::spawn(move || {
    match exec_blocking(
      &args,
      ExecOptions {
        env: Some(HashMap::from_iter(vec![(
          "APVM_PATH".to_string(),
          config.apvm_active_dir.to_str().unwrap().to_string(),
        )])),
        ..ExecOptions::default()
      },
    ) {
      Ok(_) => tx.send(Ok(())),
      Err(error) => tx.send(Err(error)),
    }
  });

  rx.await??;

  Ok(())
}
