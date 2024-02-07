# RUHear
[![Crates.io](https://img.shields.io/crates/v/ruhear)](https://crates.io/crates/ruhear)
[![Docs.rs](https://docs.rs/ruhear/badge.svg)](https://docs.rs/ruhear)
![License](https://img.shields.io/crates/l/ruhear)

A simple crate that allows you to capture <ins>system output audio</ins> (what a**R**e yo**U** **HEAR**ing).

## Dependencies
- On windows and linux: [cpal](https://crates.io/crates/cpal)
- On macos: [screencapturekit](https://crates.io/crates/screencapturekit)
- No plan for other platforms yet

## Usage
See examples folder for simple example.
```rust
use ruhear::{Ruhear, RUBuffers, RUCallback};

fn main() {
    // Create a callback that will be called every time the audio buffers are ready
    // RUBuffers is a multichannel Vec<f32>. Default sampleRate is 48000Hz on Windows and macOS and 44100Hz on Linux.
    let callback = |data: RUBuffers| {
        println!("{:?}", data);
    };

    // Create a Ruhear instance and start capturing audio, use RUCallback! macro to create a thread-safe callback
    let mut ruhear = RUCallback!(callback);

    // Start capturing audio
    ruhear.start();

    std::thread::sleep(std::time::Duration::from_secs(5));

    // Stop capturing audio
    ruhear.stop();
}
```

## TODO
- [ ] Error handling
- [ ] Add support for ASIO(Windows) and JACK(Linux)
- [ ] Add support for fine-grained control capturing audio like from a specific application/device