use crate::config::Config;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec_blocking;

// Proxy for atlaspack build
pub async fn main(config: Config) -> anyhow::Result<()> {
  let target_cli = config
    .apvm_active_dir
    .join("packages")
    .join("core")
    .join("cli")
    .join("lib")
    .join("cli.js");
  let mut args = vec!["node".to_string(), target_cli.to_str().unwrap().to_string()];
  args.extend(config.argv.into_iter());

  let (tx, rx) = tokio::sync::oneshot::channel::<anyhow::Result<()>>();

  // Run on separate thread to allow instant exit on cnt+c
  std::thread::spawn(move || match exec_blocking(&args, ExecOptions::default()) {
    Ok(_) => tx.send(Ok(())),
    Err(error) => tx.send(Err(error)),
  });

  rx.await??;

  Ok(())
}
