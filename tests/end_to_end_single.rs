extern crate mandelbrot;
extern crate png;

use mandelbrot::{complex_number::ComplexNumber, mandelbrot::{Mandelbrot, MandelbrotConfig, Viewport}, pixel::{IntoPixel, Pixel}, color_scale::{ColorScale, ContinuousColorScale, DiscreteColorScale, SimpleColorScale}};
use std::{fs::File, path::Path};
use png::Decoder;

fn flatten_array(grid: &Vec<Vec<Pixel<u8>>>) -> Vec<u8> {
    grid.iter().flat_map(|col|
        col.iter().flat_map(|pixel|
            IntoPixel::<u8>::new(pixel)))
    .collect()
}

fn vec_compare(uno: &[u8], dos: &[u8]) -> bool {
    (uno.len() == dos.len()) &&
     uno.iter()
        .zip(dos)
        .enumerate()
        .all(|(i, (a, b))| { if a != b { println!("{} != {} @ {}", a, b, i)}; a == b})
}

/// Assumes u8 for subpixels
fn single_frame_test_runner(path: &str, viewport: (ComplexNumber<f64>, f64, f64), dimensions: (u32, u32), iters: u32, color_fn: Box<Fn(u32, ComplexNumber<f64>, u32) -> Pixel<u8>>) {

    let file = File::open(Path::new(path)).unwrap();
    let decoder = Decoder::new(file);
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0u8; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let (top_left, width, height) = viewport;
    let viewport = Viewport::<f64> { top_left, width, height };
    let config = MandelbrotConfig::<u8> { dimensions, viewport, color_fn };

    let mut mandelbrot = Mandelbrot::new(config);

    mandelbrot.run_iterations(iters);
    let data = mandelbrot.get_pixels();
    let data = flatten_array(data);

    assert_eq!(buf.len(), data.len());
    println!("{:?}", buf.len());

    assert!(vec_compare(&buf, &data));
}

#[test]
fn single_frame_simple_color() {
    single_frame_test_runner("tests/assets/FHD_50_s_sc.png", (ComplexNumber::new(-3.0, 1.15), 4.0, (1080f64/1920f64) * 4f64), (1920, 1080), 50, Box::new(SimpleColorScale::pixel_color));
}

#[test]
fn single_frame_discrete_colors() {
    single_frame_test_runner("tests/assets/FHD_50_s_dc.png", (ComplexNumber::new(-3.0, 1.15), 4.0, (1080.0/1920.0) * 4.0), (1920, 1080), 50, Box::new(DiscreteColorScale::pixel_color));
}

#[test]
fn single_frame_continuous_colors() {
    single_frame_test_runner("tests/assets/FHD_50_s_cc_140_1_1.png", (ComplexNumber::new(-3.0, 1.15), 4.0, (1080.0/1920.0) * 4.0), (1920, 1080), 50, ContinuousColorScale::get_color_fn_boxed(140.0, 1.0, 1.0));
}