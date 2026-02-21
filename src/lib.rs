pub mod primitives;

#[macro_export]
macro_rules! fen {
    ($n:expr, $p:expr) => {
        $crate::primitives::field_element_num::FieldElementNum::new($n, $p)
    };
}