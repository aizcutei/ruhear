use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use ruhear::{AudioBuffers, RUHear};

fn main() {
    let callback = Arc::new(Mutex::new(Box::new(|audio_buffers: AudioBuffers| {
        println!("Channles: {:?}", audio_buffers.len());
        println!("Samples: {:?}", audio_buffers[0].len());
    })));
    let mut ruhear = RUHear::new(callback);
    ruhear.start();
    sleep(Duration::from_secs(1));
    ruhear.stop();
}
