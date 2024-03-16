pub trait Numeric {}

impl Numeric for f64 {}
impl Numeric for f32 {}
impl Numeric for i64 {}
impl Numeric for i32 {}
impl Numeric for i16 {}
impl Numeric for i8 {}
impl Numeric for isize {}
impl Numeric for u64 {}
impl Numeric for u32 {}
impl Numeric for u16 {}
impl Numeric for u8 {}
impl Numeric for usize {}
// pub trait Evaluate<T> {
//     fn evaluate(&self, x: T) -> T;
// }

// pub trait Differentiate<T> {
//     fn differentiate(&self) -> Self;
// }
