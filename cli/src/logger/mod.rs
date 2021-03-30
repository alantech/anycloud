use chrono::Utc;
use log::SetLoggerError;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CustomLogger {
  utcTime: String,
  level: String,
  env: String,
  cluster: String,
  message: String,
}

mod simple;
pub use self::simple::SimpleLogger;

mod sematext;
pub use self::sematext::Sematext;

mod logzio;
pub use self::logzio::LogzIO;

pub fn init() -> Result<(), SetLoggerError> {
  let env = std::env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
  let simple_logger = Box::new(SimpleLogger);
  let sematext = Box::new(Sematext);
  let logzio = Box::new(LogzIO);
  match env.as_str() {
    "local" => multi_log::MultiLogger::init(vec![simple_logger], log::Level::Info),
    _ => multi_log::MultiLogger::init(vec![sematext, logzio], log::Level::Info),
  }
}

pub fn create_custom_log(level: String, cluster: String, message: String) -> CustomLogger {
  let env = std::env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
  let utc_time = Utc::now().format("%FT%T%.3fZ");
  CustomLogger {
    utcTime: utc_time.to_string(),
    level: level,
    env: env,
    cluster: cluster,
    message: message,
  }
}

pub fn log_to_str(log: CustomLogger) -> Result<String, String> {
  let log_str = serde_json::to_string(&log);
  match log_str {
    Ok(log_str) => Ok(log_str),
    Err(err) => Err("Error parsing log structure".to_string()),
  }
}

pub fn log_from_str(log: String) -> Result<CustomLogger, String> {
  let custom_log = serde_json::from_str(&log);
  match custom_log {
    Ok(custom_log) => Ok(custom_log),
    Err(err) => Err("Error parsing log structure".to_string()),
  }
}
