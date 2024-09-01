extern crate pixtend;

use pixtend::PiXtend;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    pixtend.enable_safe_mode();
    pixtend.write().unwrap();
}
