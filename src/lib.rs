//! # Mandelbrot Set generator
//! The crate contains a Mandelbrot Set generator and some things needed by the generator.
//! It (along with the examples) let's you make shiny things like these:
//! 
//! ![sample][sample]
//! 
//! ## The generator:
//! If you're just trying to make Mandelbrot Sets the most important types are all in the [mandelbrot module](mandelbrot/index.html).
//! 
//! ## Using the generator:
//! Here's how it's meant to be used:
//! ```notest
//!    use mandelbrot::mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};
//!    use mandelbrot::complex_number;
//!    use mandelbrot::color_scale::SimpleColorScale;
//!    
//! 
//!    // First you need to decide how big you want to make your image:
//!    let dimensions = (1920, 1080);
//! 
//!    // Then you make a Viewport:
//!    // This tells the generator what part of the Mandelbrot Set you actually want to draw.
//!    
//!    let viewport = Viewport::<f64> {
//!        top_left: complex::ComplexNumber::new(-3.0, 1.15),
//!        width: 4f64,
//!        height: 2.25,
//!    };
//!    
//!    // Next, you can make a MandelbrotConfig struct:
//!    // This includes the Viewport and the dimensions we picked and also a Coloring Function.
//!    // The Coloring Function determines how we color in our Mandelbrot Set.
//!
//!    let config = MandelConfig::<u8> {
//!        dimensions,
//!        viewport,
//!        color_fn: Box::new(SimpleColorScale::pixel_color)
//!    }
//!    
//!    // Now, we can create a Mandelbrot struct:
//!    let mut mandelbrot = Mandelbrot::new(config);
//!    
//!    // And finally, we can run iterations:
//!    mandelbrot.run_iterations(50);
//!    
//!    // And get pixel data out of the struct:
//!    let data = mandelbrot.get_pixels();
//! 
//! ```
//! 
//! [sample]: https://raw.githubusercontent.com/rrbutani/mandelbrot/master/tests/assets/FHD_50_s_cc_140_1_1.png


pub mod color_scale;
pub mod complex_number;
pub mod mandelbrot;
pub mod pixel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
