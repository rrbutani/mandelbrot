extern crate clap;
extern crate gif;
extern crate mandelbrot;

use gif::SetParameter;
// use mandelbrot::color_scale::ColorScale;
use mandelbrot::color_scale::ContinuousColorScale;
use mandelbrot::complex_number;
use mandelbrot::mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};

mod shared;
use shared::{cli, common};

fn main() {
    let matches = cli::args().get_matches();

    let dimensions = cli::get_dimensions(&matches).expect("Invalid dimensions");
    let (w, h) = dimensions;
    let f = cli::get_number_of_frames(&matches).expect("Invalid number of frames");
    let mut file = cli::get_output_file(&matches, "a.gif").expect("Couldn't create file");

    let mut encoder = gif::Encoder::new(&mut file, w as u16, h as u16, &[]).unwrap();

    encoder.set(gif::Repeat::Infinite).unwrap();

    let viewport = Viewport::<f64> {
        top_left: complex_number::ComplexNumber::new(-3.0, 1.15),
        width: 4f64,
        height: (h as f64 / w as f64) * 4f64,
    };

    let config = MandelbrotConfig::<u8> {
        dimensions: dimensions,
        viewport: viewport,
        color_fn: ContinuousColorScale::get_color_fn_boxed(140.0, 1.0, 1.0),
        // color_fn: ContinuousColorScale::pixel_color,
        // color_fn: SimpleColorScale::pixel_color,
    };

    let mut mandelbrot = Mandelbrot::new(config);

    println!("Running {} iterations", f);

    for _i in 0..f {
        mandelbrot.run_iterations(1);
        println!("Ran an iteration");

        let pixels = mandelbrot.get_pixels();
        println!("Got the pixels");

        let frame = gif::Frame::from_rgba(w as u16, h as u16, &mut common::flatten_array(pixels));

        println!("Made a frame");

        encoder.write_frame(&frame).unwrap();

        println!("Finished writing the frame!");
    }
}
