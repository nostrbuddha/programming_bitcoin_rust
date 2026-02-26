pub const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let mut leading_zeroes = 0u32;
    for b in bytes {
        if *b == 0 {
            leading_zeroes += 1;
        } else {
            break;
        }
    }

    let mut num = bytes.to_vec();

    let mut encoded: Vec<u8> = Vec::new();

    while !num.is_empty() && num.iter().any(|&b| b != 0) {
        let mut rem: u32 = 0;

        for byte in num.iter_mut() {
            let acc = (rem << 8) | (*byte as u32);
            *byte = (acc / 58) as u8;
            rem = acc % 58;
        }

        encoded.push(BASE58_ALPHABET[rem as usize]);

        // Strip leading zeros in the quotient to keep it small
        while num.first() == Some(&0) {
            num.remove(0);
        }
    }

    // Add '1' per leading 0x00 in the original input
    for _ in 0..leading_zeroes {
        encoded.push(b'1');
    }

    encoded.reverse();
    String::from_utf8(encoded).expect("base58 output is valid utf8")
}