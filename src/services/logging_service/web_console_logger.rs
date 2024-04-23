#[derive(Clone, PartialEq)]
pub struct WebConsoleLogger;

impl WebConsoleLogger {
  pub fn new() -> Self {
    WebConsoleLogger
  }

  #[allow(dead_code)]
  pub fn error(&self, message: &str) {
    log::error!("{}", message);
  }

  #[allow(dead_code)]
  pub fn warn(&self, message: &str) {
    log::warn!("{}", message);
  }

  #[allow(dead_code)]
  pub fn info(&self, message: &str) {
    log::info!("{}", message);
  }

  #[allow(dead_code)]
  pub fn debug(&self, message: &str) {
    log::debug!("{}", message);
  }

  #[allow(dead_code)]
  pub fn trace(&self, message: &str) {
    log::trace!("{}", message);
  }
}
