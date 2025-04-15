use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct ExecOptions {
  pub cwd: Option<PathBuf>,
  pub silent: bool,
  pub env: Option<HashMap<String, String>>,
}

pub async fn exec<I, S>(
  args: I,
  options: ExecOptions,
) -> anyhow::Result<()>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut command = tokio::process::Command::new("/usr/bin/env");

  command.args(args);

  if let Some(cwd) = options.cwd {
    command.current_dir(cwd);
  }

  if let Some(extra_env) = options.env {
    for (key, val) in extra_env {
      command.env(key, val);
    }
  }

  if options.silent {
    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());
  }

  let status = command.status().await?;

  if !status.success() {
    return Err(anyhow::anyhow!("Process exited with status {}", status));
  }

  Ok(())
}

pub fn exec_blocking<I, S>(
  args: I,
  options: ExecOptions,
) -> anyhow::Result<()>
where
  I: IntoIterator<Item = S>,
  S: AsRef<OsStr>,
{
  let mut command = std::process::Command::new("/usr/bin/env");

  command.args(args);

  if let Some(cwd) = options.cwd {
    command.current_dir(cwd);
  }

  if let Some(extra_env) = options.env {
    for (key, val) in extra_env {
      command.env(key, val);
    }
  }

  if options.silent {
    command.stdout(std::process::Stdio::null());
    command.stderr(std::process::Stdio::null());
  }

  let status = command.status()?;

  if !status.success() {
    return Err(anyhow::anyhow!("Process exited with status {}", status));
  }

  Ok(())
}
