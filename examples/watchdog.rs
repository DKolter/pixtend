extern crate pixtend;

use pixtend::{PiXtend, Watchdog};

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    pixtend.set_watchdog(Watchdog::Activated8s);
    pixtend.read_write().unwrap();
}
