extern crate mandelbrot;
extern crate num_traits;

use self::num_traits::{Bounded, Unsigned, Zero};
use std::fmt::UpperHex;

use self::mandelbrot::pixel::{IntoPixel, Pixel};

pub fn flatten_array<T: Unsigned + Bounded + UpperHex + Zero + Copy>(
    grid: &Vec<Vec<Pixel<T>>>,
) -> Vec<T> {
    grid.iter()
        .flat_map(|col| col.iter().flat_map(|pixel| IntoPixel::<T>::new(pixel)))
        .collect()
}
