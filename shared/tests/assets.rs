extern crate shared;

use shared::assets::*;
use std::env;

#[test]
fn test_binary() {
    let mut asset_dir = env::current_dir().expect("Unable to find executable path.");
    asset_dir.push("tests");
    asset_dir.push("data");

    load_from_dir(asset_dir);

    if let Some(Asset::Binary(ref vec)) = get_asset("test") {
        if vec != &vec![104, 114, 114, 114, 114, 109, 10] {
            panic!("Binary data did not match test file.");
        }
    } else {
        panic!("Unable to get test file.");
    }
}

#[test]
fn test_rsi() {
    let mut asset_dir = env::current_dir().expect("Unable to find executable path.");
    asset_dir.push("tests");
    asset_dir.push("data");

    load_from_dir(asset_dir);

    if let Some(Asset::Rsi(ref rsi)) = get_asset("testrsi.rs.rsi") {
        let pass = true;
        if rsi.get_size() != (32, 32)   {
            pass = false;
        }

        let a = format!("{:?}", rsi);
        let b = "Rsi { size: (32, 32), states: {\"ByeThere\": State { full name: ByeThere, size: (32, 32), dir: 1, flags: []}, \"HiThere\": State { full name: HiThere, size: (32, 32), dir: 1, flags: []}} }";
        if a != b {
            println!("{:?}\n{:?}", a, b);
            panic!("RSI metadata incorrect.");
        }
    } else {
        panic!("Unable to get test file.");
    }
}
