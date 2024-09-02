extern crate pixtend;

use pixtend::{GpioConfig, PiXtend, SensorKind};
use std::time::Duration;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    pixtend.set_gpio_config(0, GpioConfig::Sensor).unwrap();
    pixtend.set_gpio_config(1, GpioConfig::Sensor).unwrap();
    loop {
        pixtend.read_write().unwrap();

        println!(
            "DHT11 temperature: {}",
            pixtend.get_gpio_temperature(0, SensorKind::DHT11).unwrap()
        );

        println!(
            "DHT11 humidity: {}",
            pixtend.get_gpio_humidity(0, SensorKind::DHT11).unwrap()
        );

        println!(
            "DHT22 temperature: {}",
            pixtend.get_gpio_temperature(1, SensorKind::DHT22).unwrap()
        );

        println!(
            "DHT22 humidity: {}",
            pixtend.get_gpio_humidity(1, SensorKind::DHT22).unwrap()
        );

        std::thread::sleep(Duration::from_secs(1));
    }
}
