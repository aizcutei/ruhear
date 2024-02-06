# RUHear
---
A simple crate that allows you to capture <ins>system output audio<ins> (what aRe yoU HEAR).

## Dependencies
- On windows and linux: [cpal](https://crates.io/crates/cpal)
- On macos: [screencapturekit](https://crates.io/crates/screencapturekit)

## Usage
See examples folder for simple example.
```rust
use ruhear::{Ruhear, AudioBuffers};
use std::sync::{Arc, Mutex};

fn main() {
    // Create a callback that will be called every time the audio buffers are ready
    // AudioBuffers is a multichannel Vec<f32>. Default sampleRate is 48000Hz.
    let callback = |data: AudioBuffers| {
        println!("{:?}", data);
    };

    // Create a Ruhear instance and start capturing audio, put the callback in a mutex and an arc to share it between threads
    let mut ruhear = Ruhear::new(Arc::new(Mutex::new(Box::new(callback))));

    // Start capturing audio
    ruhear.start().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));

    // Stop capturing audio
    ruhear.stop().unwrap();
}
```
## TODO
- [ ] Error handling
- [ ] Add support for fine-grained control capturing audio like from a specific application/device