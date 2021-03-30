use chrono::Utc;
use log::{Level, Metadata, Record};
use std::env;

use crate::logger::{log_from_str, CustomLogger};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= Level::Info
  }

  fn log(&self, record: &Record) {
    let log: CustomLogger = log_from_str(record.args().to_string()).unwrap();
    if self.enabled(record.metadata()) {
      if record.level() == Level::Error {
        eprintln!(
          "{} | {} | {} | {} | {}",
          log.utcTime, log.level, log.env, log.cluster, log.message
        );
      } else {
        println!(
          "{} | {} | {} | {} | {}",
          log.utcTime, log.level, log.env, log.cluster, log.message
        );
      }
    }
  }

  fn flush(&self) {}
}
