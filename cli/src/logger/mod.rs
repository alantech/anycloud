use log::{set_boxed_logger, LevelFilter, SetLoggerError};

mod simple;
pub use self::simple::SimpleLogger;

mod sematext;
pub use self::sematext::Sematext;

fn config_logger_local(_: ()) {
  log::set_max_level(LevelFilter::Trace);
}

fn config_logger(_: ()) {
  log::set_max_level(LevelFilter::Info);
}

pub fn init() -> Result<(), SetLoggerError> {
  let env = std::env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
  match env.as_str() {
    "local" => set_boxed_logger(Box::new(Sematext)).map(config_logger_local),
    _ => set_boxed_logger(Box::new(Sematext)).map(config_logger),
  }
}
