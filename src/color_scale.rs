extern crate num_traits;

use std::fmt::UpperHex;
use self::num_traits::{Unsigned, Bounded};
use pixel::{Pixel, PixelMath};

pub trait ColorScale {
    fn pixel_color<T: Unsigned + Bounded + Copy + UpperHex>(iters_to_escape: u32, num_iterations: u32) -> Pixel<T>;
}


pub struct DiscreteColorScale {}

impl ColorScale for DiscreteColorScale {
    fn pixel_color<T: Unsigned + Bounded>(_iters_to_escape: u32, _num_iterations: u32) -> Pixel<T> {
        unimplemented!()
    }
}

pub struct ContinuousColorScale {}

impl ColorScale for ContinuousColorScale {
    fn pixel_color<T: Unsigned + Bounded + Copy + UpperHex>(iters_to_escape: u32, max_iterations: u32) -> Pixel<T> {
        match (iters_to_escape as f64) / (max_iterations as f64) {
            p if p < 0.15 => Pixel::new(T::max_value(), T::min_value(), T::min_value()),
            p if p < 0.30 => Pixel::new(T::max_value(), T::max_value(), T::min_value()),
            p if p < 0.45 => Pixel::new(T::min_value(), T::max_value(), T::min_value()),
            p if p < 0.60 => Pixel::new(T::max_value(), T::max_value(), T::max_value()),
            p if p < 0.75 => Pixel::new(T::max_value(), T::min_value(), T::max_value()),
            p if p < 0.80 => Pixel::new(T::max_value(), T::min_value(), T::max_value()),
            p if p < 0.95 => Pixel::new(T::max_value(), T::max_value(), T::max_value()),
            _ => Pixel::new(T::min_value(), T::min_value(), T::min_value()),
        }
    }
}

pub struct SimpleColorScale {}

impl ColorScale for SimpleColorScale {
    fn pixel_color<T: Unsigned + Bounded + Copy + UpperHex>(iters_to_escape: u32, max_iterations: u32) -> Pixel<T> {
        if iters_to_escape == max_iterations {
            Pixel::new(T::max_value(), T::min_value(), T::min_value())
        } else {
            Pixel::new(T::min_value(), T::min_value(), T::min_value())
        }
    }
}
