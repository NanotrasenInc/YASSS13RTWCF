#[macro_use]
extern crate lazy_static;
extern crate shared;
extern crate toml;

use std::path::Path;

mod config;

fn main() {

    let mut cfg = config::CONFIG.lock().unwrap();
    cfg.load_file(Path::new("config/config.toml"));

    println!("{:?}", cfg.get_config("connection.port").expect("Unable to find port inside configuration file."));
}
