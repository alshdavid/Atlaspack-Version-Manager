use std::collections::HashMap;

use super::active::ActivePackage;
use super::exec::ExecOptions;
use super::exec::exec_blocking;
use super::runtime::resolve_runtime;
use crate::config::Config;
use crate::platform::origin::InstallOrigin;
use crate::platform::path_ext::*;

pub async fn atlaspack_exec(
  command: Vec<String>,
  config: &Config,
) -> anyhow::Result<()> {
  let runtime = resolve_runtime(&config.apvm_runtime)?;
  let Some(active) = ActivePackage::active_or_global(config)? else {
    return Err(anyhow::anyhow!("No active package selected"));
  };

  let bin_path = match active.origin {
    InstallOrigin::Super => active.static_path.join("cli").join("lib").join("cli.js"),
    InstallOrigin::Git => active
      .static_path
      .join("packages")
      .join("core")
      .join("cli")
      .join("lib")
      .join("cli.js"),
    InstallOrigin::Local => active
      .static_path
      .join("packages")
      .join("core")
      .join("cli")
      .join("lib")
      .join("cli.js"),
  };

  let mut args = Vec::<String>::new();

  args.push(runtime.try_to_string()?);
  args.push(bin_path.try_to_string()?);
  args.extend(command);

  #[rustfmt::skip]
  let env = HashMap::from_iter(vec![
    ("APVM_STATIC_PATH".to_string(), active.static_path.try_to_string()?),
    ("APVM_PATH".to_string(), active.session_path.try_to_string()?),
    ("APVM_KIND".to_string(), active.origin.to_string()),
  ]);

  let (tx, rx) = tokio::sync::oneshot::channel::<anyhow::Result<()>>();

  // Run on separate thread to allow instant exit on cnt+c
  std::thread::spawn(move || {
    match exec_blocking(
      &args,
      ExecOptions {
        env: Some(env),
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
