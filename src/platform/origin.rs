use clap::ValueEnum;
use serde::Deserialize;

#[derive(Default, PartialEq, Eq, Debug, ValueEnum, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallOrigin {
  #[default]
  Super,
  Git,
  Local,
}

impl std::fmt::Display for InstallOrigin {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let s: String = InstallOrigin::into(self.clone());
    write!(f, "{}", s)
  }
}

impl TryFrom<&str> for InstallOrigin {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "git" => Ok(Self::Git),
      "super" => Ok(Self::Super),
      "local" => Ok(Self::Local),
      _ => Err(anyhow::anyhow!("Cannot convert string to InstallOrigin")),
    }
  }
}

impl TryFrom<String> for InstallOrigin {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::try_from(value.as_str())
  }
}

impl From<InstallOrigin> for String {
  fn from(val: InstallOrigin) -> Self {
    match val {
      InstallOrigin::Super => "super",
      InstallOrigin::Git => "git",
      InstallOrigin::Local => "local",
    }
    .to_string()
  }
}
