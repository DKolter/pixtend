extern crate pixtend;

use pixtend::PiXtend;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    pixtend.read_write().unwrap();
    println!(
        "Firmware version: {}",
        pixtend.get_firmware_version().unwrap()
    );

    println!(
        "Hardware version: {}",
        pixtend.get_hardware_version().unwrap()
    );
}
