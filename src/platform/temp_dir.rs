use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

pub struct TempDir(PathBuf);

impl TempDir {
  pub fn new(target: &Path) -> Self {
    Self(target.to_path_buf())
  }
}

impl AsRef<Path> for TempDir {
  fn as_ref(&self) -> &Path {
    &self.0
  }
}

impl Deref for TempDir {
  type Target = PathBuf;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Drop for TempDir {
  fn drop(&mut self) {
    fs::remove_dir_all(&self.0).ok();
  }
}
