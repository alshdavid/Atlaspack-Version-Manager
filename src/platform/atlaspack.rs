use std::collections::HashMap;
use std::path::Path;

use super::exec::ExecOptions;
use super::exec::exec_blocking;
use super::runtime::resolve_runtime;
use crate::config::Config;
use crate::platform::path_ext::*;

pub async fn atlaspack_exec(
  command: Vec<String>,
  source_dir: &Path,
  config: &Config,
) -> anyhow::Result<()> {
  let runtime = resolve_runtime(&config.apvm_runtime)?;
  let apvm_path = config.apvm_active_dir.try_to_string()?;

  let cli_dir = source_dir.join("packages").join("core").join("cli");
  let target_entry = cli_dir.join("lib").join("cli.js");

  let mut args = Vec::<String>::new();

  args.push(runtime.try_to_string()?);
  args.push(target_entry.try_to_string()?);
  args.extend(command);

  let (tx, rx) = tokio::sync::oneshot::channel::<anyhow::Result<()>>();

  // Run on separate thread to allow instant exit on cnt+c
  std::thread::spawn(move || {
    match exec_blocking(
      &args,
      ExecOptions {
        env: Some(HashMap::from_iter(vec![(
          "APVM_PATH".to_string(),
          apvm_path,
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
