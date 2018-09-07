extern crate mandelbrot;
extern crate num_traits;

use std::fmt::UpperHex;
use self::num_traits::{Bounded, Unsigned, Zero};

use self::mandelbrot::pixel::{Pixel, IntoPixel};
// use mandelbrot::pixel;

// pub fn flatten_array(grid: Vec<Vec<Vec<u8>>>) -> Vec<u8> {
//     grid.iter().flat_map(|col|
//         col.iter().flat_map(|pixel|
//             pixel.iter()))
//             //             // .iter()
//             //             // .flat_map(|pixel| pixel.iter())
//             //         )
//             // )
//     .cloned()
//     .collect()
// }

pub fn flatten_array<T: Unsigned + Bounded + UpperHex + Zero + Copy>(grid: &Vec<Vec<Pixel<T>>>) -> Vec<T> {
    // let temp: Vec<Vec<Vec<T>>> = grid.iter().map(|col|
    //     col.iter().map(|px|
    //         px.get_vector()))
    // .collect().collect();

    // temp.flatten();

    // grid.iter().flat_map(|col|
    //     col.iter().flat_map(|pixel|
    //         pixel.get_vector().iter()))
    // .cloned()
    // .collect()

    grid.iter().flat_map(|col|
        col.iter().flat_map(|pixel|
            IntoPixel::<T>::new(pixel)))
    // .cloned()
    .collect()
}
