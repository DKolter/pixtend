extern crate pixtend;

use pixtend::{Channel, PiXtend};
use std::time::Duration;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    loop {
        for value in [0.0, 5.0, 10.0] {
            pixtend.set_analog_output(Channel::A, Some(value));
            pixtend.set_analog_output(Channel::B, Some(value));
            pixtend.read_write().unwrap();
            std::thread::sleep(Duration::from_secs(5));
        }
    }
}
