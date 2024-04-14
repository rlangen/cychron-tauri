use uuid::{NoContext, Timestamp};

use super::logging_service::Log;

pub struct UuidService;

impl UuidService {
    /// Returns a UUIDv7 index
    /// It is a UUIDv7 and not a usize or a string, because the creation of a UUID doesn't need an API call nor a singleton like counter.
    /// An API call is slow and a singleton like counter is harder to initialize and maintain.
    pub fn new_index() -> u128 {
        let id: u128;
        let now = Self::now_seconds_and_miliseconds();
        match now {
            Some((seconds, nanos)) => {
                let ts = Timestamp::from_unix(NoContext, seconds, nanos);
                id = uuid::Uuid::new_v7(ts).as_u128();
            }
            None => {
                Log::warn::<UuidService>("Failed to get current time. Using random UUID instead.");
                id = uuid::Uuid::new_v4().as_u128();
            }
        }

        id
    }

    fn now_seconds_and_miliseconds() -> Option<(u64, u32)> {
        let seconds;
        let nanos;

        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                let now = performance.now();
                seconds = (now / 1000.0) as u64;
                nanos = ((now % 1000.0) * 1_000_000.0) as u32;
            } else {
                return None;
            }
        } else {
            return None;
        }
        Some((seconds, nanos))
    }
}
