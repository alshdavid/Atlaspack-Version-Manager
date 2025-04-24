#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub static TARBALL: &str = "atlaspack-darwin-arm64.tar.gz";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub static TARBALL: &str = "atlaspack-darwin-amd64.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub static TARBALL: &str = "atlaspack-linux-arm64.tar.gz";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub static TARBALL: &str = "atlaspack-linux-amd64.tar.gz";

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub static TARBALL: &str = "atlaspack-windows-arm64.tar.gz";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub static TARBALL: &str = "atlaspack-windows-amd64.tar.gz";
