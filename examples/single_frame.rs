extern crate clap;
extern crate png;
extern crate mandelbrot;

use mandelbrot::mandelbrot::MandelbrotConfig;
use mandelbrot::mandelbrot::Mandelbrot;
use mandelbrot::mandelbrot::Viewport;
use png::HasParameters;

mod shared;
use shared::{cli, common};
use std::io::BufWriter;

use mandelbrot::{pixel::{Pixel, PixelMath, IntoPixel}, complex_number};

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
        _pixel: Pixel::<u8>::default(),
    };

    let mut mandelbrot = Mandelbrot::new(config);

    mandelbrot.run_iterations(50);

    let data = mandelbrot.get_pixels();

    // let mut data = vec![vec![vec![ 0 as u8; 4]; w as usize]; h as usize];

    // data[(w - w) as usize][(h - h) as usize][0] = 255;
    // data[(w - w) as usize][(h - h) as usize][1] = 255;
    // data[(w - w) as usize][(h - h) as usize][2] = 0;
    // data[(w - w) as usize][(h - h) as usize][3] = 255;



    writer.write_image_data(common::flatten_array(data).as_slice()).unwrap();

    println!("{:?}, {:?}", w, h);

    let px = Pixel::new(9u8, 234, 5);

    let iter = IntoPixel::new(&px);

    for i in iter {
        println!("{:?}", i);
    }
}
