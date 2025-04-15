use std::path::PathBuf;

pub struct Env {
  pub apvm_session: Option<String>,
  pub apvm_dir: PathBuf,
  pub apvm_local: Option<PathBuf>,
}

impl Env {
  pub fn parse() -> anyhow::Result<Self> {
    Ok(Self {
      apvm_dir: match std::env::var("APVM_DIR") {
        Ok(apvm_dir) => PathBuf::from(apvm_dir),
        Err(_) => apvm_dir_default()?,
      },
      apvm_session: std::env::var("APVM_SESSION").ok(),
      apvm_local: match std::env::var("APVM_LOCAL") {
        Ok(apvm_local) => Some(PathBuf::from(apvm_local)),
        Err(_) => None,
      },
    })
  }
}

fn apvm_dir_default() -> anyhow::Result<PathBuf> {
  let Ok(Some(current_exe)) = homedir::my_home() else {
    return Err(anyhow::anyhow!(
      "Cannot find apvm_home. Please set $APVM_HOME variable manually"
    ));
  };
  let default_dir = current_exe.join(".local").join("apvm").join("apvm_dir");
  if default_dir.is_file() {
    return Err(anyhow::anyhow!("{:?} exists but is a file", current_exe));
  }
  if !default_dir.exists() {
    std::fs::create_dir_all(&default_dir)?;
  }
  Ok(default_dir)
}
