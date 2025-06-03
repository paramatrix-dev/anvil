/// Convert any number into an f64.
pub trait IntoF64 {
    /// Convert this number into an f64
    fn to_f64(&self) -> f64;
}
macro_rules! impl_into_f64 {
    ($t:tt) => {
        impl IntoF64 for $t {
            fn to_f64(&self) -> f64 {
                *self as f64
            }
        }
    };
}

impl_into_f64!(usize);
impl_into_f64!(isize);
impl_into_f64!(u8);
impl_into_f64!(u16);
impl_into_f64!(u32);
impl_into_f64!(u64);
impl_into_f64!(u128);
impl_into_f64!(i8);
impl_into_f64!(i16);
impl_into_f64!(i32);
impl_into_f64!(i64);
impl_into_f64!(i128);
impl_into_f64!(f32);
impl_into_f64!(f64);
