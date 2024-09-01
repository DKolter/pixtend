use crate::error::PiXtendError;
use deku::prelude::*;

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Retain {
    #[deku(count = "64")]
    pub storage: Vec<u8>,
}

impl Retain {
    pub fn set_retain_data(&mut self, mut data: Vec<u8>) -> Result<(), PiXtendError> {
        match data.len() {
            0..=64 => {
                data.resize(64, 0);
                self.storage = data;
            }
            _ => return Err(PiXtendError::InvalidRetainDataLength(data.len())),
        }

        Ok(())
    }
}

impl Default for Retain {
    fn default() -> Self {
        Self {
            storage: vec![0; 64],
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
