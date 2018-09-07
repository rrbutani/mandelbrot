extern crate num_traits;

use std::fmt::Debug;
use std::fmt::UpperHex;
use pixel::{Pixel, PixelMath};

use color_scale::SimpleColorScale;
use color_scale::ColorScale;
use self::num_traits::{Float, Bounded, Zero};
use self::num_traits::sign::Unsigned;

use std::convert::From;

use ::complex_number::ComplexNumber;

pub struct Viewport<T: Float> 
{
    pub top_left: ComplexNumber<T>,
    pub width: T,
    pub height: T,
}

pub struct MandelbrotConfig<P: Unsigned + Bounded + UpperHex + Copy + Zero> {
    pub dimensions: (u32, u32),
    pub viewport: Viewport<f64>,
    pub _pixel: Pixel<P>,
}

pub struct Mandelbrot<P: Unsigned + Bounded + UpperHex + Copy + Zero> {
    config: MandelbrotConfig<P>,
    pixels: Vec<Vec<Pixel<P>>>,
    values: Vec<Vec<(u32, ComplexNumber<f64>)>>,
    steps: (ComplexNumber<f64>, ComplexNumber<f64>),
    iterations: u32,
}

impl<P: Unsigned + Bounded + UpperHex + Copy + Zero> Mandelbrot<P> {
    pub fn new(config: MandelbrotConfig<P>) -> Mandelbrot<P> {
        let (w, h) = config.dimensions;

        let w_c = ComplexNumber::new(config.viewport.width, 0.0);
        let h_c = ComplexNumber::new(0.0, -config.viewport.height);

        // println!("Viewport width is {} -> {:?}; width is {}; therefore step size is {:?}", config.viewport.width, w_c, w, w_c/w);

        Mandelbrot {
            config: config,
            pixels: vec![vec![Pixel::<P>::default(); w as usize]; h as usize],
            values: vec![vec![(0, ComplexNumber::new(0.0, 0.0)); w as usize]; h as usize],
            steps: (w_c / w, h_c / h),
            iterations: 0,
        }
    }

    pub fn get_pixels(&self) -> &Vec<Vec<Pixel<P>>> {
        &self.pixels
    }

    pub fn run_iterations(&mut self, num_iters: u32) {
        let (w, h) = self.config.dimensions;
        let (d_w, d_h) = self.steps;

        let mut coordinate = self.config.viewport.top_left;

        // println!("We ready! Starting at {:?} and stepping {:?} and then {:?}", coordinate, d_w, d_h);

        self.iterations += num_iters;

        for r in 0..(h as usize) {
            let mut coordinate2 = coordinate;

            for c in 0..(w as usize) {
                // println!("@ {}, {}", r, c);
                self.values[r][c] = iterate_coordinate(self.values[r][c], coordinate2, num_iters);

                let iters_to_escape = self.values[r][c].0;
                self.pixels[r][c] = SimpleColorScale::pixel_color(iters_to_escape, self.iterations);

                coordinate2 = coordinate2 + d_w;
            }

            coordinate = coordinate + d_h;
        }

    }
}

fn iterate_coordinate<T: Float + Debug>(current_coord: (u32, ComplexNumber<T>), c: ComplexNumber<T>, limit: u32) -> (u32, ComplexNumber<T>)
    where f64: From<T> {
    let mut count = 0;
    let (finished_iters, mut z) = current_coord;
    let two = ComplexNumber::<f64>::new(2.0, 0.0);

    // println!("Got {:?} and {:?} ({:?}) at {} iterations", c, z, z.abs(), finished_iters);

    while two > z && count < limit {
        z = z * z + c;
        count += 1;
        // println!("    z is {:?} after {} iterations", z, count);
    }

    (count + finished_iters, z)
}
