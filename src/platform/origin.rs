use super::apvmrc::ApvmRc;

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

  pub fn resolve(
    apvmrc: &Option<ApvmRc>,
    version: &Option<String>,
  ) -> anyhow::Result<Self> {
    let version = version.clone().unwrap_or_default();
    let apvmrc = apvmrc.clone().unwrap_or_default();

    // Order of target selection:
    // (empty version)  -> package.json#atlaspack.version
    // version          -> package.json#atlaspack.versions[version]
    // version          -> lookup (git|npm|local)[version]

    // If the version is empty and there is a default specified in the apvmrc
    if version.is_empty() {
      if let Some(target) = apvmrc.version_target {
        return Ok(target.clone());
      }
      return Err(anyhow::anyhow!("No version selected"));
    }

    // If the version is specified and it matches an alias in the apvmrc
    if let Some(target) = apvmrc.version_target_aliases.get(&version) {
      return Ok(target.clone());
    }

    // Resolve the version specified
    Self::try_from(version.as_str())
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
