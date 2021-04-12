
pub trait MinLimit {
    fn min() -> Self;
}

pub trait MaxLimit {
    fn max() -> Self;
}

macro_rules! min_limit_impl  {
    ($type:ident) => {
        impl MinLimit for $type {
            fn min() -> $type { std::$type::MIN }
        }
    };
}

macro_rules! max_limit_impl  {
    ($type:ident) => {
        impl MaxLimit for $type {
            fn max() -> $type { std::$type::MAX }
        }
    };
}

macro_rules! numeric_limit_impl  {
    ($type:ident) => {
        min_limit_impl !($type);
        max_limit_impl !($type);
    };
}

numeric_limit_impl!(f32);
numeric_limit_impl!(f64);
numeric_limit_impl!(u32);
numeric_limit_impl!(u64);
numeric_limit_impl!(i32);
numeric_limit_impl!(i64);
