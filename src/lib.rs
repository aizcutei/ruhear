use std::sync::{Arc, Mutex};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::RUHear;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::RUHear;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::RUHear;

pub type RUBuffers = Vec<Vec<f32>>;

#[macro_export]
macro_rules! RUCallback {
    ($callback:expr) => {
        Arc::new(Mutex::new($callback))
    };
}
