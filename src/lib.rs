pub mod primitives;
pub mod s256;

#[macro_export]
macro_rules! fen {
    ($n:expr, $p:expr) => {
        $crate::primitives::field_element_num::FieldElementNum::new($n, $p)
    };
}