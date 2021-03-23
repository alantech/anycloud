use log::SetLoggerError;

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
    "local" => multi_log::MultiLogger::init(vec![simple_logger, sematext, logzio], log::Level::Info),
    _ => multi_log::MultiLogger::init(vec![sematext], log::Level::Info),
  }
}
