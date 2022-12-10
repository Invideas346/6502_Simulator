use std::ops::{Add, Sub};

pub struct Register<T: Copy + Add<Output = T> + Sub<Output = T>> {
    pub value: T,
}

impl<T: Copy + Add<Output = T> + Sub<Output = T>> Register<T> {
    pub fn new(value: T) -> Register<T> {
        Register { value }
    }
}
