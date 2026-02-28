use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

pub fn hash160(data: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(data);
    let ripe = Ripemd160::digest(sha);
    ripe.into()
}