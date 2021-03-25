use chrono::Utc;
use log::{Level, Metadata, Record};
use std::env;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    let utc_time = Utc::now().format("%FT%T%.3fZ");
    let env = env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
    let cluster = record.target(); // If no target defined default to `anycloud`
    if self.enabled(record.metadata()) {
      if record.level() == Level::Error {
        eprintln!(
          "{} | {} | {} | {} | {}",
          utc_time,
          record.level(),
          env,
          cluster,
          record.args()
        );
      } else {
        println!(
          "{} | {} | {} | {} | {}",
          utc_time,
          record.level(),
          env,
          cluster,
          record.args()
        );
      }
    }
  }

  fn flush(&self) {}
}
