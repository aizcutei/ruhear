#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use ruhear::{rucallback, RUBuffers, RUHear};

fn main() {
    let callback = |audio_buffers: RUBuffers| {
        println!("Channles: {:?}", audio_buffers.len());
        println!("Samples: {:?}", audio_buffers[0].len());
    };
    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback);
    ruhear.start();
    sleep(Duration::from_secs(1));
    ruhear.stop();
}
