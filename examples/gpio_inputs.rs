extern crate pixtend;

use pixtend::{GpioConfig, PiXtend};
use std::time::Duration;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    for i in 0..=3 {
        pixtend
            .set_gpio_config(i, GpioConfig::Input(false))
            .unwrap();
    }

    loop {
        pixtend.read_write().unwrap();
        for i in 0..=3 {
            println!("Gpio input {}: {}", i, pixtend.get_gpio_input(i).unwrap());
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}