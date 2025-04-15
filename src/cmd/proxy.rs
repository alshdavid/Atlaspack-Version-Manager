use crate::config::Config;
use crate::platform::atlaspack::atlaspack_exec;

// Proxy for atlaspack build
pub async fn main(config: Config) -> anyhow::Result<()> {
  if !std::fs::exists(&config.apvm_active_dir)? {
    return Err(anyhow::anyhow!("No active version installed"));
  }

  let link = std::fs::read_link(config.apvm_active_dir.join("static"))?;
  atlaspack_exec(config.argv.clone(), &link, &config).await?;
  Ok(())
}
