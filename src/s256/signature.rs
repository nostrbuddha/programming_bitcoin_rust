use crate::s256::s256_field::S256Field;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Signature {
    pub r: S256Field,
    pub s: S256Field,
}
