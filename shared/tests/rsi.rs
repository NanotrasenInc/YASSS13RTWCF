extern crate shared;
extern crate image;

use image::{open, GenericImage};
use shared::rsi::Rsi;
use std::path::Path;

#[test]
fn test_extract() {
    let path = Path::new("tests/data/testrsi.rs.rsi");
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

#[test]
fn test_delay() {
    let path = Path::new("tests/data/testrsi.rs.rsi");
    let rsi = Rsi::open(path).unwrap();

    let byethere = rsi.get("ByeThere").unwrap();

    for index in 0..4 {
        assert_eq!(byethere.get_delay(0, index).unwrap(), 1.0);
    }
}
