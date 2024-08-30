extern crate pixtend;

use pixtend::PiXtend;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    loop {
        for i in 0..=11 {
            pixtend.set_digital_output(i, true).unwrap();
            pixtend.write().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        for i in 0..=11 {
            pixtend.set_digital_output(i, false).unwrap();
            pixtend.write().unwrap();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
