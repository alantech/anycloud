use log::{set_boxed_logger, LevelFilter, SetLoggerError};

mod simple;
pub use self::simple::SimpleLogger;

mod sematext;
pub use self::sematext::Sematext;

pub fn init() -> Result<(), SetLoggerError> {
  let env = std::env::var("ALAN_TECH_ENV").unwrap_or("production".to_string());
  let sematext = Box::new(Sematext);
  let simple_logger = Box::new(SimpleLogger);
  match env.as_str() {
    "local" => multi_log::MultiLogger::init(vec![simple_logger, sematext], log::Level::Info),
    _ => multi_log::MultiLogger::init(vec![sematext], log::Level::Info),
  }
}
