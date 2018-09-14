//! Contains the `Mandelbrot` type and some friends

extern crate num_traits;

use pixel::{Pixel, PixelMath};
use std::fmt::{Debug, UpperHex};

use self::num_traits::{sign::Unsigned, Bounded, Float, Zero};

use std::{cmp, convert::From};

use complex_number::ComplexNumber;

/// This represents the section of the complex plane that we're going to
/// calculate values (`ComplexNumber`) and generate pixels (`Pixel`) for.
///
/// `top_left` sets the coordinate to start at, width determines the real
/// part values to cover, and height determines the imaginary part values to
/// cover.
pub struct Viewport<T: Float> {
    /// The top left coordinate for the grid that is to be plotted
    pub top_left: ComplexNumber<T>,
    /// Width of the grid
    pub width: T,
    /// Height of the grid
    pub height: T,
}

/// This struct provides all the settings needed for the `Mandelbrot` type to go
/// calculate values and pixels.
///
/// Ultimately, based on the dimensions and the `Viewport` provided, discretized
/// coordinates are calculated for each pixel and (after iterations are run) the
/// coloring function given is used to color in each pixel.
pub struct MandelbrotConfig<P: Unsigned + Bounded + UpperHex + Copy + Zero> {
    /// The pixel dimensions of the area to generate values/pixels for
    pub dimensions: (u32, u32),
    /// The `Viewport` to cover
    pub viewport: Viewport<f64>,
    /// The (Boxed) coloring function to be used
    pub color_fn: Box<Fn(u32, ComplexNumber<f64>, u32) -> Pixel<P>>,
}

pub struct Mandelbrot<P: Unsigned + Bounded + UpperHex + Copy + Zero> {
    config: MandelbrotConfig<P>,
    pixels: Vec<Vec<Pixel<P>>>,
    values: Vec<Vec<(u32, ComplexNumber<f64>)>>,
    steps: (ComplexNumber<f64>, ComplexNumber<f64>),
    iterations: u32,
}

impl<P: 'static + Unsigned + Bounded + UpperHex + Copy + Zero + Into<f64>> Mandelbrot<P> {
    pub fn new(config: MandelbrotConfig<P>) -> Mandelbrot<P> {
        let (w, h) = config.dimensions;

        let w_c = ComplexNumber::new(config.viewport.width, 0.0);
        let h_c = ComplexNumber::new(0.0, -config.viewport.height);

        Mandelbrot {
            config,
            pixels: vec![vec![Pixel::<P>::default(); w as usize]; h as usize],
            values: vec![vec![(0, ComplexNumber::new(0.0, 0.0)); w as usize]; h as usize],
            steps: (w_c / w, h_c / h),
            iterations: 0,
        }
    }

    /// Returns a reference to the current state of the Pixels in the
    /// Mandelbrot Set
    pub fn get_pixels(&self) -> &Vec<Vec<Pixel<P>>> {
        &self.pixels
    }

    /// Runs the number of iterations given across all the Pixels in the
    /// Mandelbrot Set
    pub fn run_iterations(&mut self, num_iters: u32) {
        let (w, h) = self.config.dimensions;
        let (d_w, d_h) = self.steps;
        let coordinate = self.config.viewport.top_left;

        self.iterations += num_iters;

        let mut max_iterations: u32 = 0;

        for r in 0..(h as usize) {
            for c in 0..(w as usize) {
                self.values[r][c] = iterate_coordinate(
                    self.values[r][c],
                    coordinate + d_w * (c as f64) + d_h * (r as f64),
                    num_iters,
                );

                max_iterations = cmp::max(max_iterations, self.values[r][c].0);
            }
        }

        for (r, row) in self.values.iter().enumerate() {
            for (c, (iters, zn)) in row.iter().enumerate() {
                self.pixels[r][c] = (self.config.color_fn)(*iters, *zn, max_iterations);
            }
        }
    }

    pub fn reset(&mut self) {
        self.pixels
            .iter_mut()
            .map(|col| col.iter_mut().map(|px| *px = Pixel::<P>::default()))
            .count();
        self.values
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|coor| *coor = (0, ComplexNumber::new(0.0, 0.0)))
            }).count();
        self.iterations = 0;
    }
}

/// A helper function that runs the number of iterations given on a single
/// coordinate
fn iterate_coordinate<T: Float + Debug>(
    current_coord: (u32, ComplexNumber<T>),
    c: ComplexNumber<T>,
    limit: u32,
) -> (u32, ComplexNumber<T>)
where
    f64: From<T>,
{
    let mut count = 0;
    let (finished_iters, mut z) = current_coord;
    let two = ComplexNumber::<f64>::new(2.0, 0.0);

    while two > z && count < limit {
        z = z * z + c;
        count += 1;
    }

    (count + finished_iters, z)
}
