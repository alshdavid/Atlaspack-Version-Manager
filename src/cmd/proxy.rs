use crate::config::Config;
use crate::platform::atlaspack::atlaspack_exec;

// Proxy for atlaspack build
pub async fn main(config: Config) -> anyhow::Result<()> {
  atlaspack_exec(config.argv.clone(), &config).await?;
  Ok(())
}
