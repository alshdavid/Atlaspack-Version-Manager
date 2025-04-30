use crate::context::Context;
use crate::platform::atlaspack::atlaspack_exec;

// Proxy for atlaspack build
pub fn main(ctx: Context) -> anyhow::Result<()> {
  atlaspack_exec(&ctx, ctx.env.argv.clone())?;
  Ok(())
}
