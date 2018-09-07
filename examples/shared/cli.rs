// #[macro_use]
extern crate clap;
use clap::{App, Arg, AppSettings, ArgMatches};
use std::fs::File;
use std::path::Path;
use std::io::Result;

macro_rules! env_or {
    ($env_var:expr, $fallback:expr) => (option_env!($env_var).unwrap_or($fallback))
}

// #[allow(unused_macros)]
// macro_rules! cargo_env {
//     ($cargo_env_var:expr) => (env!(stringify!(CARGO_$cargo_env_var), stringify!(Please set $cargo_env_var in Cargo.toml)))
// }

macro_rules! cargo_env {
    ($cargo_env_var:ident) => (env!(concat!("CARGO_", stringify!($cargo_env_var))))
}


// fn env_or(cargo_env_var: &'static str, fallback: &'static str) -> &'static str {
//     option_env!(cargo_env_var).unwrap_or_else(fallback)
// }

// pub fn args2<'a>(_a: &'a i32) -> &'a ArgMatches {
//     let matches = App::new(env_or!("CARGO_PKG_NAME", "Unknown"))
//             .version(env_or!("CARGO_PKG_VERSION", "Unknown"))
//             .author(env_or!("CARGO_PKG_AUTHORS", "Unknown"))
//             .about(env_or!("CARGO_PKG_DESCRIPTION", "???"))
//             .get_matches();

//     &matches
// }

#[allow(dead_code)]
pub fn args_alt() -> App<'static, 'static> {
    App::new(env_or!("CARGO_PKG_NAME", "Unknown"))
            .version(env_or!("CARGO_PKG_VERSION", "Unknown"))
            .author(env_or!("CARGO_PKG_AUTHORS", "Unknown"))
            .about(env_or!("CARGO_PKG_DESCRIPTION", "???"))
}

pub fn args() -> App<'static, 'static> {
    App::new(cargo_env!(PKG_NAME))
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(cargo_env!(PKG_VERSION))
        .author(cargo_env!(PKG_AUTHORS))
        .about(cargo_env!(PKG_DESCRIPTION))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_names(&["FILE"])
            .number_of_values(1))
        .arg(Arg::with_name("generate")
            .short("g")
            .long("generate")
            .value_names(&["width", "height"])
            .number_of_values(2)
            .requires("output"))
        .arg(Arg::with_name("frames")
            .short("f")
            .long("frames")
            .value_names(&["number of frames"])
            .number_of_values(1)
            .requires("generate"))
}

pub fn get_dimensions(m: &ArgMatches) -> Option<(u32, u32)> {
    if let Some(vals) = m.values_of("generate") {
        let dim: Vec<&str> = vals.collect();

        match (dim[0].parse::<u32>(), dim[1].parse::<u32>()) {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => None,
        }
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn get_number_of_frames(m: &ArgMatches) -> Option<u32> {
    if let Some(val) = m.value_of("frames") {
        match val.parse::<u32>() {
            Ok(a) => Some(a),
            _ => None,
        }
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn get_output_file(m: &ArgMatches, def: &str) -> Result<File> {
    let path_str = m.value_of("output").unwrap_or(def);
    let path = Path::new(path_str);
    File::create(path)
}

// pub fn args<'a, 'b, 'c>(args: &'a mut App<'b, 'c>) -> App<'b, 'c> {
//     args.name(env_or!("CARGO_PKG_NAME", "Unknown"))
//             .version(env_or!("CARGO_PKG_VERSION", "Unknown"))
//             .author(env_or!("CARGO_PKG_AUTHORS", "Unknown"))
//             .about(env_or!("CARGO_PKG_DESCRIPTION", "???"))

// }

// pub fn args_alt() -> clap::ArgMatches {
//     clap_app!()
// }