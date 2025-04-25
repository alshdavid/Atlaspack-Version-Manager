// use std::collections::HashMap;
// use std::path::PathBuf;

// use super::active::ActivePackage;
// use super::exec::ExecOptions;
// use super::exec::exec_blocking;
// use super::runtime::resolve_runtime;
// use crate::config::Config;
// use crate::platform::origin::VersionTarget;
// use crate::platform::path_ext::*;

// pub async fn atlaspack_exec(
//   command: Vec<String>,
//   config: &Config,
// ) -> anyhow::Result<()> {
//   let runtime = resolve_runtime(&config.apvm_runtime)?;
//   let Some(active) = ActivePackage::active_or_global(config)? else {
//     return Err(anyhow::anyhow!("No active package selected"));
//   };

//   let bin_path = match detect_node_modules(config) {
//     // Use entry point from node_modules if available
//     Some(bin_path) => bin_path,
//     // Otherwise use entry point from currently active Atlaspack
//     None => match active.origin {
//       VersionTarget::Npm => active.static_path.join("cli").join("lib").join("cli.js"),
//       VersionTarget::Git => active
//         .static_path_real
//         .join("packages")
//         .join("core")
//         .join("cli")
//         .join("lib")
//         .join("cli.js"),
//       VersionTarget::Local => active
//         .static_path_real
//         .join("packages")
//         .join("core")
//         .join("cli")
//         .join("lib")
//         .join("cli.js"),
//     },
//   };

//   let mut args = Vec::<String>::new();

//   args.push(runtime.try_to_string()?);
//   args.push(bin_path.try_to_string()?);
//   args.extend(command);

//   #[rustfmt::skip]
//   let env = HashMap::from_iter(vec![
//     ("APVM_STATIC_PATH".to_string(), active.static_path.try_to_string()?),
//     ("APVM_PATH".to_string(), active.session_path.try_to_string()?),
//     ("APVM_KIND".to_string(), active.origin.to_string()),
//   ]);

//   let (tx, rx) = tokio::sync::oneshot::channel::<anyhow::Result<()>>();

//   // Run on separate thread to allow instant exit on cnt+c
//   std::thread::spawn(move || {
//     match exec_blocking(
//       &args,
//       ExecOptions {
//         env: Some(env),
//         ..ExecOptions::default()
//       },
//     ) {
//       Ok(_) => tx.send(Ok(())),
//       Err(error) => tx.send(Err(error)),
//     }
//   });

//   rx.await??;
//   Ok(())
// }

// fn detect_node_modules(config: &Config) -> Option<PathBuf> {
//   let node_modules_bin = config.exe_path.parent()?;
//   if !node_modules_bin.ends_with(".bin") {
//     return None;
//   }
//   let node_modules = node_modules_bin.parent()?;
//   if !node_modules.ends_with("node_modules") {
//     return None;
//   }
//   Some(
//     node_modules
//       .join("@atlaspack")
//       .join("cli")
//       .join("lib")
//       .join("cli.js"),
//   )
// }
