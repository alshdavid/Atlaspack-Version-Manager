#![allow(non_upper_case_globals)]

use crate::config::Config;

const color_red: &str = "\x1B[31m";
const color_reset: &str = "\x1B[39m";
const style_bold: &str = "\x1B[1m";
const style_reset: &str = "\x1B[0m";

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[rustfmt::skip]
pub async fn main(_config: Config) -> anyhow::Result<()> {
  print!("{}", color_red);
  println!(r#"      __         _______  ___      ___  ___      ___ "#);
  println!(r#"     /""\       |   __ "\|"  \    /"  ||"  \    /"  |"#);
  println!(r#"    /    \      (. |__) :)\   \  //  /  \   \  //   |"#);
  println!(r#"   /' /\  \     |:  ____/  \\  \/. ./   /\\  \/.    |"#);
  println!(r#"  //  __'  \    (|  /       \.    //   |: \.        |"#);
  println!(r#" /   /  \\  \  /|__/ \       \\   /    |.  \    /:  |"#);
  println!(r#"(___/    \___)(_______)       \__/     |___|\__/|___|"#);
  println!(r#"                                                     "#);
  print!("{}", color_reset);
  print!("{}", style_bold);
  println!(r#"           Atlaspack Version Manager {}              "#, VERSION);
  print!("{}", style_reset);
  Ok(())
}
