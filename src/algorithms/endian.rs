use crypto_bigint::U256;


pub fn  little_endian_to_int(bytes: &[u8]) -> U256 {
    let mut buf = [0u8; 32];
    assert!(bytes.len() <= 32, "integer too large for U256");
    buf[..bytes.len()].copy_from_slice(bytes);
    U256::from_le_slice(&buf)

    /*
    let mut result = U256::ZERO;

    for (i, &byte) in bytes.iter().enumerate() {
        let value = U256::from(byte) << (8 * i);
        result += value;
    }

    result
    */
}

pub fn int_to_little_endian(n: U256, length: usize) -> Vec<u8> {
    let mut bytes = n.to_le_bytes().to_vec();
    bytes.truncate(length);
    bytes
}