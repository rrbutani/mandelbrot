//! # Mandelbrot Set generator
//! The crate contains a Mandelbrot Set generator and some things needed by the
//! generator.
//! It (along with the examples) let's you make shiny things like these:
//!
//! ![sample][sample]
//!
//! ## The modules:
//! If you're just trying to make Mandelbrot Sets the most important types are
//! all in the [`mandelbrot`](mandelbrot/index.html) module.
//!
//! If you're trying to get a little bit deeper:
//!   - the [`color_scale`](color_scale/index.html) module has everything
//!     related to coloring in the Mandelbrot Set
//!   - the [`complex_number`](complex_number/index.html) module has the
//!     implementations for the operators we use on `ComplexNumber` values in
//!     this library
//!   - and finally, the [`pixel`](pixel/index.html) module has everything
//!     relevant to pixels in this library
//!
//! ## Using the `Mandelbrot` Type:
//! Here's how it's meant to be used:
//! ```rust
//! use mandelbrot::{
//!     color_scale::{ColorScale, SimpleColorScale},
//!     complex_number::ComplexNumber,
//!     mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport},
//! };
//!
//! // First you need to decide how big you want to make your image:
//! let dimensions = (1920, 1080);
//!
//! // Then you make a Viewport:
//! // This tells the generator what part of the Mandelbrot Set you actually
//! // want to draw.
//!
//! let viewport = Viewport::<f64> {
//!     top_left: ComplexNumber::new(-3.0, 1.15),
//!     width: 4f64,
//!     height: 2.25,
//! };
//!
//! // Next, you can make a MandelbrotConfig struct:
//! // This includes the Viewport and the dimensions we picked and also a
//! // Coloring Function. The Coloring Function determines how we color in
//! // our Mandelbrot Set.
//!
//! let config = MandelbrotConfig::<u8> {
//!     dimensions,
//!     viewport,
//!     color_fn: Box::new(SimpleColorScale::pixel_color),
//! };
//!
//! // Now, we can create a Mandelbrot struct:
//! let mut mandelbrot = Mandelbrot::new(config);
//!
//! // And finally, we can run iterations:
//! mandelbrot.run_iterations(50);
//!
//! // And get pixel data out of the struct:
//! let data = mandelbrot.get_pixels();
//! ```
//!
//! [sample]: https://raw.githubusercontent.com/rrbutani/mandelbrot/master/tests/assets/FHD_50_s_cc_140_1_1.png

pub mod color_scale;
pub mod complex_number;
pub mod mandelbrot;
pub mod pixel;

#[cfg(test)]
mod tests {

    use color_scale::{ColorScale, SimpleColorScale};
    use complex_number::ComplexNumber;
    use mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};

    #[test]
    fn altogether_now() {
        let dimensions = (1920, 1080);
        let viewport = Viewport::<f64> {
            top_left: ComplexNumber::new(-3.0, 1.15),
            width: 4f64,
            height: 2.25,
        };

        // We're going to use _really_ big pixels for fun!
        let config = MandelbrotConfig::<u32> {
            dimensions,
            viewport,
            color_fn: Box::new(SimpleColorScale::pixel_color),
        };

        let mut mandelbrot = Mandelbrot::new(config);
        mandelbrot.run_iterations(50);
        let data = mandelbrot.get_pixels();

        assert_eq!(data.len(), 1080);
        assert_eq!(data[0].len(), 1920);
    }
}
