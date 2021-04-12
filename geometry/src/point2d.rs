use std::ops;

use base::{MinLimit, MaxLimit};
use std::ops::{Add, AddAssign};

#[derive(Clone)]
pub struct Point<T: Default + MinLimit + MaxLimit> {
    x: T,
    y: T
}

impl<T: Default + MinLimit + MaxLimit> Point<T> {
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

impl <'a> AddAssign<&'a Point<i32>> for Point<i32> {

    fn add_assign(&mut self, other: &'a Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}