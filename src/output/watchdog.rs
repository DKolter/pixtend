use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite, PartialEq, Eq, Default)]
#[deku(id_type = "u8")]
pub enum Watchdog {
    #[default]
    #[deku(id = "0")]
    Deactivated,
    #[deku(id = "1")]
    Activated16ms,
    #[deku(id = "2")]
    Activated32ms,
    #[deku(id = "3")]
    Activated64ms,
    #[deku(id = "4")]
    Activated0_125s,
    #[deku(id = "5")]
    Activated0_25s,
    #[deku(id = "6")]
    Activated0_5s,
    #[deku(id = "7")]
    Activated1s,
    #[deku(id = "8")]
    Activated2s,
    #[deku(id = "9")]
    Activated4s,
    #[deku(id = "10")]
    Activated8s,
}

#[test]
fn test_watchdog_control() {
    let data = [0];
    let (_, control) = Watchdog::from_bytes((&data, 0)).unwrap();
    assert_eq!(control, Watchdog::Deactivated);

    let data = [4];
    let (_, control) = Watchdog::from_bytes((&data, 0)).unwrap();
    assert_eq!(control, Watchdog::Activated0_125s);

    let data = [7];
    let (_, control) = Watchdog::from_bytes((&data, 0)).unwrap();
    assert_eq!(control, Watchdog::Activated1s);

    let data = [10];
    let (_, control) = Watchdog::from_bytes((&data, 0)).unwrap();
    assert_eq!(control, Watchdog::Activated8s);

    let control = Watchdog::Deactivated;
    let data = control.to_bytes().unwrap();
    assert_eq!(data, [0]);

    let control = Watchdog::Activated0_125s;
    let data = control.to_bytes().unwrap();
    assert_eq!(data, [4]);
}
