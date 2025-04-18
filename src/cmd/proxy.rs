use crate::config::Config;
use crate::platform::atlaspack::atlaspack_exec;

// Proxy for atlaspack build
pub async fn main(config: Config) -> anyhow::Result<()> {
  if !std::fs::exists(&config.apvm_active_dir)? {
    return Err(anyhow::anyhow!("No active version installed"));
  }

  atlaspack_exec(config.argv.clone(), &config).await?;
  Ok(())
}
