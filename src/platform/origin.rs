use clap::ValueEnum;

#[derive(PartialEq, Eq, Debug, ValueEnum, Clone)]
pub enum InstallOrigin {
  Super,
  Git,
  Local,
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
