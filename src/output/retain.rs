use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Retain {
    #[deku(count = "64")]
    pub storage: Vec<u8>,
}

impl Default for Retain {
    fn default() -> Self {
        Self {
            storage: vec![0x00; 64],
        }
    }
}

#[test]
fn test_retain() {
    let data = vec![0x01; 64];
    let (_, retain) = Retain::from_bytes((&data, 0)).unwrap();
    assert_eq!(retain.storage, data);
    assert_eq!(retain.to_bytes().unwrap(), data);

    assert_eq!(Retain::default().to_bytes().unwrap(), vec![0x00; 64]);
}
