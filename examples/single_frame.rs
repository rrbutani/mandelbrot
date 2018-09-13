extern crate clap;
extern crate mandelbrot;
extern crate png;

use mandelbrot::mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport};
use png::HasParameters;

mod shared;
use shared::{cli, common};
use std::io::BufWriter;

use mandelbrot::complex_number;

#[allow(unused_imports)]
use mandelbrot::color_scale::{ColorScale, ContinuousColorScale, SimpleColorScale};

fn main() {
    let matches = cli::args().get_matches();

    let dimensions = cli::get_dimensions(&matches).expect("Invalid dimensions");
    let (w, h) = dimensions;
    let file = cli::get_output_file(&matches, "a.png").expect("Couldn't create file");

    let ref mut buf = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf, w, h);
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let viewport = Viewport::<f64> {
        top_left: complex_number::ComplexNumber::new(-3.0, 1.15),
        width: 4f64,
        height: (h as f64 / w as f64) * 4f64,
    };

    let config = MandelbrotConfig::<u8> {
        dimensions: dimensions,
        viewport: viewport,
        // color_fn: ContinuousColorScale::get_color_fn(20.0, 0.8, 1.0),
        color_fn: ContinuousColorScale::get_color_fn_boxed(200.0, 1.0, 1.0),
        // color_fn: Box::new(SimpleColorScale::pixel_color),
    };

    let mut mandelbrot = Mandelbrot::new(config);

    mandelbrot.run_iterations(50);

    let data = mandelbrot.get_pixels();

    writer
        .write_image_data(common::flatten_array(data).as_slice())
        .unwrap();
}
