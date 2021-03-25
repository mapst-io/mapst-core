use base::{Min, Max};

#[derive(Clone)]
pub struct Point<T: Default + Min + Max> {
    x: T,
    y: T
}

impl<T: Default + Min + Max> Point<T> {
    pub fn zero() -> Point<T> {
        return Point { x: T::default(), y: T::default() }
    }

    pub fn max() -> Point<T> {
        return Point { x: T::max(), y: T::max() }
    }

    pub fn new(x: T, y: T) -> Point<T> {
        return Point { x, y }
    }
}