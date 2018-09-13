extern crate mandelbrot;
extern crate num_traits;

use std::fmt::UpperHex;
use self::num_traits::{Bounded, Unsigned, Zero};

use self::mandelbrot::pixel::{Pixel, IntoPixel};

pub fn flatten_array<T: Unsigned + Bounded + UpperHex + Zero + Copy>(grid: &Vec<Vec<Pixel<T>>>) -> Vec<T> {
    grid.iter().flat_map(|col|
        col.iter().flat_map(|pixel|
            IntoPixel::<T>::new(pixel)))
    .collect()

    // grid.into_iter().flatten().collect()
}

