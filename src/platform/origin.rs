use clap::ValueEnum;

#[derive(PartialEq, Eq, Debug, ValueEnum, Clone)]
pub enum InstallOrigin {
  Super,
  Git,
  Local,
}

impl std::fmt::Display for InstallOrigin {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "{}", self.to_string())
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

impl Into<String> for InstallOrigin {
  fn into(self) -> String {
    match self {
      InstallOrigin::Super => "super",
      InstallOrigin::Git => "git",
      InstallOrigin::Local => "local",
    }
    .to_string()
  }
}
