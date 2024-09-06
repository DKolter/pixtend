<div align="center">

# PiXtend

<img src="https://shop.pixtend.de/images/product_images/original_images/pixtend_v2_l_eplc_pro_angeschlossen_1.png" width="400px"/>

</div>

## Features

* Support for the Raspberry PiXtend L
* Safe API, which prevents configuration mistakes by design with good error handling
* Reading digital inputs, analog inputs with automatic unit conversion, DHT11 and DHT22 sensors via GPIOs
* Writing digital outputs, GPIO, relays, analog outputs via DAC
* Reading and writing of retain memory supported
* Safemode and watchdog settings

## Example

```rust
use pixtend::PiXtend;

fn main() {
    let mut pixtend = PiXtend::new().unwrap();
    for i in 0..=11 {
        pixtend.set_digital_output(i, true).unwrap();
        pixtend.read_write().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

```

## Planned

* PiXtend S support
* Extension boards (EIO digital / analog)
