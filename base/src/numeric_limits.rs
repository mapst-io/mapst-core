
pub trait Min {
    fn min() -> Self;
}

pub trait Max {
    fn max() -> Self;
}

impl Min for f32 {
    fn min() -> f32 { std::f32::MIN }
}

impl Max for f32 {
    fn max() -> f32 { std::f32::MAX }
}

impl Min for f64 {
    fn min() -> f64 { std::f64::MIN }
}

impl Max for f64 {
    fn max() -> f64 { std::f64::MAX }
}

impl Min for u32 {
    fn min() -> u32 { std::u32::MIN }
}

impl Max for u32 {
    fn max() -> u32 { std::u32::MAX }
}

impl Min for u64 {
    fn min() -> u64 { std::u64::MIN }
}

impl Max for u64 {
    fn max() -> u64 { std::u64::MAX }
}

impl Min for i32 {
    fn min() -> i32 { std::i32::MIN }
}

impl Max for i32 {
    fn max() -> i32 { std::i32::MAX }
}

impl Min for i64 {
    fn min() -> i64 { std::i64::MIN }
}

impl Max for i64 {
    fn max() -> i64 { std::i64::MAX }
}
