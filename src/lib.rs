//! Capture audio from the system and send it to a callback function.
//!
//! See [examples](https://github.com/aizcutei/ruhear/tree/main/examples) for usage.

#[cfg(not(target_os = "macos"))]
mod cpal;
#[cfg(not(target_os = "macos"))]
pub use cpal::RUHear;

#[cfg(target_os = "macos")]
mod screencapturekit;
#[cfg(target_os = "macos")]
pub use screencapturekit::RUHear;

pub type RUBuffers = Vec<Vec<f32>>;

#[macro_export]
macro_rules! rucallback {
    ($callback:expr) => {
        Arc::new(Mutex::new($callback))
    };
}
