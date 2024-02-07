use ruhear::{rucallback, RUBuffers, RUHear};
use std::sync::{Arc, Mutex};

fn main() {
    let callback = |audio: RUBuffers| {
        println!("{:?}", audio);
    };
    let callback = rucallback!(callback);
    let mut ruhear = RUHear::new(callback.clone());

    let result = ruhear.stop();
    print!("{:?}", result);
}
