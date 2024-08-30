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
