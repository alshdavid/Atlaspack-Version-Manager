use std::fs;

use crate::config::Config;

pub async fn main(config: Config) -> anyhow::Result<()> {
  fs::remove_file(config.apvm_active_dir)?;
  Ok(())
}
