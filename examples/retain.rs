extern crate pixtend;

use pixtend::PiXtend;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    pixtend.set_retain_enable(true);
    pixtend.set_retain_data(vec![1, 2, 3, 4, 5]).unwrap();
    pixtend.read_write().unwrap();
    println!("{:?}", pixtend.get_retain_data().unwrap());
}
