#![allow(dead_code)]

use std::time::Instant;

/// A utility for profiling
pub struct Stopwatch {
    instant_start: Instant,
}

impl Stopwatch {
    /// Starts the stopwatch
    pub fn start() -> Self {
        Self {
            instant_start: Instant::now(),
        }
    }

    /// Gives the time elapsed in seconds
    pub fn elapsed(&self) -> f32 {
        (Instant::now() - self.instant_start).as_secs_f32()
    }
}
