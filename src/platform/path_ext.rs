#![allow(unused)]

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

pub trait OsStringExt {
  fn try_to_string(self) -> anyhow::Result<String>;
}

impl OsStringExt for OsString {
  fn try_to_string(self) -> anyhow::Result<String> {
    match self.into_string() {
      Ok(name) => Ok(name),
      Err(_) => Err(anyhow::anyhow!("Unable to convert OsString to String")),
    }
  }
}

impl OsStringExt for &OsStr {
  fn try_to_string(self) -> anyhow::Result<String> {
    match self.to_str() {
      Some(name) => Ok(name.to_string()),
      None => Err(anyhow::anyhow!("Unable to convert OsString to String")),
    }
  }
}

pub trait PathExt {
  fn try_parent(&self) -> anyhow::Result<&Path>;
  fn try_file_name(&self) -> anyhow::Result<String>;
  fn try_file_stem(&self) -> anyhow::Result<String>;
  fn try_to_string(&self) -> anyhow::Result<String>;
}

impl PathExt for PathBuf {
  fn try_parent(&self) -> anyhow::Result<&Path> {
    match self.parent() {
      Some(path) => Ok(path),
      None => Err(anyhow::anyhow!("Unable to find parent")),
    }
  }

  fn try_file_name(&self) -> anyhow::Result<String> {
    match self.file_name() {
      Some(v) => Ok(v.try_to_string()?),
      None => Err(anyhow::anyhow!("Cannot get file name")),
    }
  }

  fn try_file_stem(&self) -> anyhow::Result<String> {
    match self.file_stem() {
      Some(v) => Ok(v.try_to_string()?),
      None => Err(anyhow::anyhow!("Cannot get file stem")),
    }
  }

  fn try_to_string(&self) -> anyhow::Result<String> {
    match self.to_str() {
      Some(v) => Ok(v.to_string()),
      None => Err(anyhow::anyhow!("Cannot convert Path to string")),
    }
  }
}

impl PathExt for Path {
  fn try_parent(&self) -> anyhow::Result<&Path> {
    match self.parent() {
      Some(path) => Ok(path),
      None => Err(anyhow::anyhow!("Unable to find parent")),
    }
  }

  fn try_file_name(&self) -> anyhow::Result<String> {
    match self.file_name() {
      Some(v) => Ok(v.try_to_string()?),
      None => Err(anyhow::anyhow!("Cannot get file name")),
    }
  }

  fn try_file_stem(&self) -> anyhow::Result<String> {
    match self.file_stem() {
      Some(v) => Ok(v.try_to_string()?),
      None => Err(anyhow::anyhow!("Cannot get file stem")),
    }
  }

  fn try_to_string(&self) -> anyhow::Result<String> {
    match self.to_str() {
      Some(v) => Ok(v.to_string()),
      None => Err(anyhow::anyhow!("Cannot convert Path to string")),
    }
  }
}
