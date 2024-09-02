pub fn calc_crc16(data: impl Iterator<Item = u8>) -> u16 {
    let mut crc = 0xFFFF;
    for byte in data {
        crc ^= byte as u16;

        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}
