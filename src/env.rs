use std::path::PathBuf;

pub struct Env {
  pub apvm_session: Option<String>,
  pub apvm_runtime: String,
  pub apvm_dir: PathBuf,
}

impl Env {
  pub fn parse() -> anyhow::Result<Self> {
    Ok(Self {
      apvm_dir: match std::env::var("APVM_DIR") {
        Ok(apvm_dir) => PathBuf::from(apvm_dir),
        Err(_) => apvm_dir_default()?,
      },
      apvm_runtime: match std::env::var("APVM_RUNTIME") {
        Ok(apvm_runtime) => apvm_runtime,
        Err(_) => "node".to_string(),
      },
      apvm_session: std::env::var("APVM_SESSION").ok(),
    })
  }
}

fn apvm_dir_default() -> anyhow::Result<PathBuf> {
  let Ok(Some(current_exe)) = homedir::my_home() else {
    return Err(anyhow::anyhow!(
      "Cannot find apvm_home. Please set $APVM_HOME variable manually"
    ));
  };
  let default_dir = current_exe.join(".local").join("apvm");
  if default_dir.is_file() {
    return Err(anyhow::anyhow!("{:?} exists but is a file", current_exe));
  }
  if !default_dir.exists() {
    std::fs::create_dir_all(&default_dir)?;
  }
  Ok(default_dir)
}
