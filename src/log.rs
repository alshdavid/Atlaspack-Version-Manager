use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Level {
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}
