use std::path::Path;

pub fn link(
  original: &Path,
  link: &Path,
) -> anyhow::Result<()> {
  #[cfg(unix)]
  std::os::unix::fs::symlink(original, link)?;

  #[cfg(windows)]
  std::os::windows::fs::symlink_dir(original, link)?;

  Ok(())
}
