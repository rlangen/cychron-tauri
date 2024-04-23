use once_cell::sync::Lazy;
use std::any::type_name;

mod web_console_logger;
use web_console_logger::WebConsoleLogger;

static LOG: Lazy<WebConsoleLogger> = Lazy::new(|| WebConsoleLogger::new());

pub struct Log;
impl Log {
  const SEPERATOR: &'static str = ": ";
  const SEPERATOR_LEN: usize = Self::SEPERATOR.len();

  #[allow(dead_code)]
  pub fn error<T: ?Sized>(message: &str) {
    LOG.error(Self::concat_type_and_message::<T>(message).as_str());
  }

  #[allow(dead_code)]
  pub fn warn<T: ?Sized>(message: &str) {
    LOG.warn(Self::concat_type_and_message::<T>(message).as_str());
  }

  #[allow(dead_code)]
  pub fn info<T: ?Sized>(message: &str) {
    LOG.info(Self::concat_type_and_message::<T>(message).as_str());
  }

  #[allow(dead_code)]
  #[cfg(debug_assertions)]
  pub fn debug<T: ?Sized>(message: &str) {
    LOG.debug(Self::concat_type_and_message::<T>(message).as_str());
  }

  #[allow(dead_code)]
  #[cfg(debug_assertions)]
  pub fn trace<T: ?Sized>(message: &str) {
    LOG.trace(Self::concat_type_and_message::<T>(message).as_str());
  }

  fn concat_type_and_message<T: ?Sized>(message: &str) -> String {
    let type_string = type_name::<T>();

    let mut result = String::with_capacity(type_string.len() + Self::SEPERATOR_LEN + message.len());

    result.push_str(type_string);
    result.push_str(Self::SEPERATOR);
    result.push_str(message);

    result
  }
}
