#[derive(PartialEq, Eq, Debug, Clone)]
pub enum VersionTarget {
  Npm(String),
  Git(String),
  Local(String),
}

impl std::fmt::Display for VersionTarget {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let s: String = VersionTarget::into(self.clone());
    write!(f, "{s}")
  }
}

impl TryFrom<&str> for VersionTarget {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let Some((origin, specifier)) = value.split_once(":") else {
      return Ok(Self::Npm(value.to_string()));
    };
    match origin {
      "git" => Ok(Self::Git(specifier.to_string())),
      "npm" => Ok(Self::Npm(specifier.to_string())),
      "local" => Ok(Self::Local(specifier.to_string())),
      _ => Err(anyhow::anyhow!("Cannot convert string to VersionTarget")),
    }
  }
}

impl TryFrom<String> for VersionTarget {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Self::try_from(value.as_str())
  }
}

impl From<VersionTarget> for String {
  fn from(val: VersionTarget) -> Self {
    match val {
      VersionTarget::Npm(version) => version.to_string(),
      VersionTarget::Git(version) => format!("git:{}", version),
      VersionTarget::Local(version) => version.to_string(),
    }
  }
}

impl VersionTarget {
  pub fn parse<S: AsRef<str>>(value: S) -> anyhow::Result<Self> {
    Self::try_from(value.as_ref())
  }

  pub fn version(&self) -> &str {
    match self {
      VersionTarget::Npm(version) => version,
      VersionTarget::Git(version) => version,
      VersionTarget::Local(version) => version,
    }
  }

  pub fn origin(&self) -> &str {
    match self {
      VersionTarget::Npm(_version) => "npm",
      VersionTarget::Git(_version) => "git",
      VersionTarget::Local(_version) => "local",
    }
  }
}
