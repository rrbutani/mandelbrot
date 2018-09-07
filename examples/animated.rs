extern crate clap;

mod shared;
use shared::cli;

fn main() {
    let matches = cli::args().get_matches();

    let (w, h) = cli::get_dimensions(&matches).expect("Invalid dimensions");
    let f = cli::get_number_of_frames(&matches).expect("Invalid number of frames");

    println!("{:?}, {:?}", w, h);
    println!("{:?}", f);
}
