use crate::{error::PiXtendError, GpioConfig};
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, Default)]
pub struct GpioCtrl {
    #[deku(bits = "1")]
    pub sens3: bool,
    #[deku(bits = "1")]
    pub sens2: bool,
    #[deku(bits = "1")]
    pub sens1: bool,
    #[deku(bits = "1")]
    pub sens0: bool,
    #[deku(bits = "1")]
    pub io3: bool,
    #[deku(bits = "1")]
    pub io2: bool,
    #[deku(bits = "1")]
    pub io1: bool,
    #[deku(bits = "1")]
    pub io0: bool,
}

impl GpioCtrl {
    pub fn set_gpio_config(&mut self, index: u8, config: GpioConfig) -> Result<(), PiXtendError> {
        if index > 3 {
            return Err(PiXtendError::InvalidGpioOutputIndex(index));
        }

        let ios = [&mut self.io0, &mut self.io1, &mut self.io2, &mut self.io3];
        *ios[index as usize] = matches!(config, GpioConfig::Output);

        let sensors = [
            &mut self.sens0,
            &mut self.sens1,
            &mut self.sens2,
            &mut self.sens3,
        ];
        *sensors[index as usize] = matches!(config, GpioConfig::Sensor);

        Ok(())
    }
}

#[test]
fn test_gpio_ctrl() {
    let data = vec![0b1010_1010];
    let (_, gpio_ctrl) = GpioCtrl::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(gpio_ctrl.sens3, true);
    assert_eq!(gpio_ctrl.sens2, false);
    assert_eq!(gpio_ctrl.sens1, true);
    assert_eq!(gpio_ctrl.sens0, false);
    assert_eq!(gpio_ctrl.io3, true);
    assert_eq!(gpio_ctrl.io2, false);
    assert_eq!(gpio_ctrl.io1, true);
    assert_eq!(gpio_ctrl.io0, false);
    assert_eq!(gpio_ctrl.to_bytes().unwrap(), data);

    let data = vec![0b0101_0101];
    let (_, gpio_ctrl) = GpioCtrl::from_bytes((data.as_ref(), 0)).unwrap();
    assert_eq!(gpio_ctrl.sens3, false);
    assert_eq!(gpio_ctrl.sens2, true);
    assert_eq!(gpio_ctrl.sens1, false);
    assert_eq!(gpio_ctrl.sens0, true);
    assert_eq!(gpio_ctrl.io3, false);
    assert_eq!(gpio_ctrl.io2, true);
    assert_eq!(gpio_ctrl.io1, false);
    assert_eq!(gpio_ctrl.io0, true);
    assert_eq!(gpio_ctrl.to_bytes().unwrap(), data);
}
