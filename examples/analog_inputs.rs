extern crate pixtend;

use pixtend::{PiXtend, ReferenceVoltage};
use std::time::Duration;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    loop {
        pixtend.read_write().unwrap();
        for i in 0..=3 {
            println!(
                "Analog voltage input {}: {}V",
                i,
                pixtend
                    .get_analog_voltage_input(i, ReferenceVoltage::V10)
                    .unwrap()
            );
        }

        for i in 4..=5 {
            println!(
                "Analog current input {}: {}A",
                i,
                pixtend.get_analog_current_input(i).unwrap()
            );
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
