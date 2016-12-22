#[macro_use]
extern crate lazy_static;
extern crate shared;
extern crate toml;

use std::path::Path;
use shared::config;

// uuuuh....
#[allow(dead_code)]
fn main() {
    let mut cfg = config::CONFIG.lock().unwrap();
    cfg.load_file(Path::new("config/config.toml")).unwrap();

    println!("{:?}", cfg.get("connection.port").expect("Unable to find port inside configuration file."));
}
