extern crate pixtend;

use pixtend::PiXtend;
use std::time::Duration;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    loop {
        pixtend.read_write().unwrap();
        for i in 0..=15 {
            println!(
                "Digital input {}: {}",
                i,
                pixtend.get_digital_input(i).unwrap()
            );
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}
