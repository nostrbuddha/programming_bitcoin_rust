pub mod primitives;
pub mod s256;

#[macro_export]
macro_rules! fe32 {
    ($n:expr, $p:expr) => {
        $crate::primitives::field_element_u32::FieldElementU32::new($n, $p)
    };
}

#[macro_export]
macro_rules! p32 {
    ($n:expr, $p:expr, $a:expr, $b:expr) => {
        $crate::primitives::point_i32::PointI32::new(Some($n), Some($p), $a, $b)
    };
}

#[macro_export]
macro_rules! p_fe32 {
    ($n:expr, $p:expr, $a:expr, $b:expr) => {
        $crate::primitives::point_fe_u32::PointFEU32::new(Some($n), Some($p), $a, $b)
    };
}
#[macro_export]
macro_rules! fe {
    ($n:expr, $p:expr) => {
        $crate::primitives::field_element::FieldElement::new($n, $p)
    };
}