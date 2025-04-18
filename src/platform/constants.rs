#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub static TARBALL: &str = "atlaspack-darwin-arm64.tar.xz";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
pub static TARBALL: &str = "atlaspack-darwin-x64.tar.xz";

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
pub static TARBALL: &str = "atlaspack-linux-arm64.tar.xz";

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub static TARBALL: &str = "atlaspack-linux-x64.tar.xz";

#[cfg(all(target_os = "windows", target_arch = "aarch64"))]
pub static SUTARBALLFFIX: &str = "atlaspack-windows-arm64.tar.xz";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub static TARBALL: &str = "atlaspack-windows-x64.tar.xz";
