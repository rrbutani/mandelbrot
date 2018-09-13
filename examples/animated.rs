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
    let f = cli::get_number_of_frames(&matches).expect("Invalid number of frames");
    let file = cli::get_output_file(&matches, "a.png").expect("Couldn't create file");

    let ref mut buf = BufWriter::new(file);

    let mut encoder = png::Encoder::new_animated(buf, w, h, f + 1).unwrap();
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
        color_fn: ContinuousColorScale::get_color_fn_boxed(140.0, 1.0, 1.0),
        // color_fn: ContinuousColorScale::pixel_color,
    };

    println!("Running {} iterations", f);

    let mut mandelbrot = Mandelbrot::new(config);
    mandelbrot.run_iterations(f);

    writer
        .write_frame(common::flatten_array(mandelbrot.get_pixels()).as_slice())
        .unwrap();

    mandelbrot.reset();

    for _i in 0..f {
        mandelbrot.run_iterations(1);
        let data = common::flatten_array(mandelbrot.get_pixels());
        let data = data.as_slice();

        writer.write_frame(data).unwrap();
    }
}
