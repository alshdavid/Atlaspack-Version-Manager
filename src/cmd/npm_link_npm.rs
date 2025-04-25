use std::fs;
use std::fs::Permissions;
use std::path::Path;

use fs_extra::dir::CopyOptions;
use json::JsonValue;

use super::npm_link::NpmLinkCommand;
use crate::config::Config;
use crate::platform::exec::ExecOptions;
use crate::platform::exec::exec_blocking;
use crate::platform::package::PackageDescriptor;
use crate::platform::path_ext::*;
use crate::platform::temp_dir::TempDir;

pub async fn npm_link_npm(
  config: Config,
  cmd: NpmLinkCommand,
  package: PackageDescriptor,
) -> anyhow::Result<()> {
  let package_lib = package.path_real()?;
  let package_lib_static = package_lib.join("lib");

  // node_modules
  let node_modules = config.pwd.join("node_modules");
  let node_modules_bin = node_modules.join(".bin");

  // node_modules/.bin
  let node_modules_bin_atlaspack = node_modules_bin.join("atlaspack");

  // node_modules/atlaspack
  let node_modules_super = node_modules.join("atlaspack");
  let node_modules_temp = node_modules.join(".apvm");

  // node_modules/@atlaspack
  let node_modules_atlaspack = node_modules.join("@atlaspack");

  // Create node_modules if it doesn't exist
  if !fs::exists(&node_modules)? {
    fs::create_dir_all(&node_modules)?;
  }
  if !fs::exists(&node_modules_bin)? {
    fs::create_dir_all(&node_modules_bin)?;
  }

  // Delete existing node_modules/.bin/atlaspack
  if fs::exists(&node_modules_bin_atlaspack)? {
    fs::remove_file(&node_modules_bin_atlaspack)?;
  }

  // Delete existing node_modules/.apvm
  if fs::exists(&node_modules_temp)? {
    fs::remove_dir_all(&node_modules_temp)?;
  }
  fs::create_dir_all(&node_modules_temp)?;
  fs::write(
    node_modules_temp.join("version"),
    format!("{}", package.version_target),
  )?;

  // node_modules/atlaspack
  if fs::exists(&node_modules_super)? {
    fs::remove_dir_all(&node_modules_super)?;
  }

  // TEMP reconstruct package
  create_atlaspack_super(&config, &package, &package_lib, &node_modules)?;

  #[cfg(unix)]
  {
    use std::os::unix::fs::PermissionsExt;

    fs::write(
      &node_modules_bin_atlaspack,
      "#!/usr/bin/env node\nrequire('atlaspack/cli.js')\n",
    )?;
    fs::set_permissions(&node_modules_bin_atlaspack, Permissions::from_mode(0o777))?;
  }

  if !cmd.legacy {
    return Ok(());
  }

  // Create node_modules/@atlaspack
  if fs::exists(&node_modules_atlaspack)? {
    fs::remove_dir_all(&node_modules_atlaspack)?
  }
  fs::create_dir_all(&node_modules_atlaspack)?;

  for entry in fs::read_dir(&package_lib_static)? {
    let entry = entry?;
    let entry_path = entry.path();

    if fs::metadata(&entry_path)?.is_dir() {
      continue;
    }

    let file_stem = entry_path.try_file_stem()?;

    if file_stem.starts_with("vendor.") {
      continue;
    }

    let node_modules_atlaspack_pkg = node_modules_atlaspack.join(&file_stem);
    if fs::exists(&node_modules_atlaspack_pkg)? {
      fs::remove_dir_all(&node_modules_atlaspack_pkg)?;
    }

    fs::create_dir(&node_modules_atlaspack_pkg)?;
    fs::write(
      node_modules_atlaspack_pkg.join("package.json"),
      (json::object! {
        "name": format!("@atlaspack/{file_stem}"),
        "main": "./index.js"
      })
      .to_string(),
    )?;

    fs::write(
      node_modules_atlaspack_pkg.join("index.js"),
      format!("module.exports = require('atlaspack/{file_stem}.js')\n"),
    )?;
  }

  Ok(())
}

/// Temporary function that builds the public interface of the require('atlaspack') package
/// e.g.
/// require('atlaspack/core')
/// require('atlaspack/fs')
/// require('atlaspack/fs')
fn create_atlaspack_super(
  config: &Config,
  package: &PackageDescriptor,
  package_root: &Path,
  node_modules: &Path,
) -> anyhow::Result<()> {
  let temp = TempDir::new(&config.paths.temp)?;
  let package_lib = package_root.join("lib");

  fs_extra::dir::copy(
    &package_lib,
    &temp,
    &CopyOptions {
      content_only: true,
      ..Default::default()
    },
  )?;

  // Install node_modules and merge with old
  fs::create_dir_all(temp.join("_node_modules"))?;

  fs_extra::dir::move_dir(
    temp.join("node_modules"),
    temp.join("_node_modules"),
    &CopyOptions {
      content_only: true,
      ..Default::default()
    },
  )?;

  exec_blocking(
    [
      "npm",
      "install",
      "detect-libc",
      "@parcel/watcher",
      "posthtml",
      "cosmiconfig",
    ],
    ExecOptions {
      cwd: Some(temp.to_path_buf()),
      silent: true,
      env: None,
    },
  )?;

  let restore = vec![
    "@atlaspack/rust",
    "htmlnano",
    "lightningcss",
    "lightningcss-linux-x64-gnu",
    "lightningcss-linux-x64-musl",
    "lightningcss-wasm",
    "@parcel/source-map",
    "@swc",
  ];
  for entry in restore {
    if !fs::exists(temp.join("node_modules").join(entry))? {
      fs::create_dir_all(temp.join("node_modules").join(entry))?;
    }
    fs_extra::dir::move_dir(
      temp.join("_node_modules").join(entry),
      temp.join("node_modules").join(entry),
      &CopyOptions {
        content_only: true,
        ..Default::default()
      },
    )?;
  }

  // Generate package.json
  let package_json_exports = json::object! {
    ".": "./core.js",
    "./*": "./*",
  };
  let JsonValue::Object(mut package_json_exports) = package_json_exports else {
    panic!()
  };

  for entry in fs::read_dir(&temp)? {
    let entry = entry?;
    let entry_path = entry.path();

    if fs::metadata(&entry_path)?.is_dir() {
      continue;
    }

    let file_stem = entry_path.try_file_stem()?;
    let file_name = entry_path.try_file_name()?;

    if file_stem.starts_with("vendor.") {
      continue;
    }

    if !file_name.ends_with(".js") {
      continue;
    }

    package_json_exports.insert(
      &format!("./{}", file_stem),
      JsonValue::from(format!("./{}", file_name)),
    );
  }

  fs::write(
    temp.join("package.json"),
    (json::object! {
      "name": "atlaspack",
      "version": format!("{}", package.version),
      "exports": package_json_exports
    })
    .pretty(2),
  )?;

  // Copy into dest
  fs::remove_dir_all(temp.join("_node_modules"))?;
  fs::create_dir_all(temp.join(node_modules.join("atlaspack")))?;

  fs_extra::dir::copy(
    &temp,
    node_modules.join("atlaspack"),
    &CopyOptions {
      content_only: true,
      ..Default::default()
    },
  )?;

  Ok(())
}
