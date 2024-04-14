use once_cell::sync::Lazy;
use std::any::type_name;

mod web_console_logger;
use web_console_logger::WebConsoleLogger;

static LOG: Lazy<WebConsoleLogger> = Lazy::new(|| WebConsoleLogger::new());

pub struct Log;
impl Log {
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
        format!("{}: {}", type_name::<T>(), message)
    }
}
