extern crate shared;

use shared::assets::*;
use std::env;

#[test]
fn test_binary() {
    let mut asset_dir = env::current_exe().expect("Unable to find executable path.");
    asset_dir.pop();
    asset_dir.push("data");

    print!("{:?}", asset_dir);

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
    let mut asset_dir = env::current_exe().expect("Unable to find executable path.");
    asset_dir.pop();
    asset_dir.push("data");

    print!("{:?}", asset_dir);

    load_from_dir(asset_dir);

    if let Some(Asset::Rsi(ref rsi)) = get_asset("testrsi.rs.rsi") {
        if format!("{:?}", rsi) != "Rsi { size: (32, 32), states: {\"ByeThere\": State { full name: ByeThere, size: (32, 32), dir: 1, flags: []}, \"HiThere\": State { full name: HiThere, size: (32, 32), dir: 1, flags: []}} }" {
            panic!("RSI metadata incorrect.");
        }
    } else {
        panic!("Unable to get test file.");
    }
}
