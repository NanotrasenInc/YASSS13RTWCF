extern crate shared;
extern crate image;

use image::{open, GenericImage};
use shared::rsi::Rsi;
use std::path::Path;
use std::env;

#[test]
fn testrsi() {
    let path = Path::new("tests/data/testrsi.rs.rsi");
    println!("{:?}", env::current_dir());

    assert!(path.is_dir());

    let rsi = Rsi::open(path).unwrap();

    let otherbyethere = open("tests/data/testrsi.rs_byethere_3.png").unwrap();

    let byethere = rsi.get("ByeThere").unwrap();

    let icon = byethere.get_icon(0, 2).unwrap();

    for pixel in icon.pixels().zip(otherbyethere.pixels()) {
        if pixel.0 != pixel.1 {
            panic!(format!("Pixel incorrect: {:?}", pixel));
        }
    }
}
