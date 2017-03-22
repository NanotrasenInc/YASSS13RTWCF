extern crate shared;
extern crate image;

use image::DynamicImage;
use shared::assets::*;
use shared::rsi::Rsi;
use std::env;

#[test]
fn test_binary() {
    let mut asset_dir = env::current_dir().expect("Unable to find executable path.");
    asset_dir.push("tests");
    asset_dir.push("data");

    load_from_dir(asset_dir).unwrap();

    let asset = get_asset("test").expect("Unable to get test file.");
    if let Asset::Binary(ref vec) = *asset {
        assert_eq!(vec, &vec![104, 114, 114, 114, 114, 109]);
    } else {
        panic!("Asset is not binary!")
    }
}

#[test]
fn test_rsi() {
    let mut asset_dir = env::current_dir().expect("Unable to find executable path.");
    asset_dir.push("tests");
    asset_dir.push("data");

    load_from_dir(asset_dir).unwrap();

    let asset = get_asset("testrsi.rs.rsi").expect("Unable to get test file.");
    let rsi = asset.as_rsi().expect("Asset is not an RSI.");
    assert_eq!(rsi.get_size(), (32, 32));

    let mut tester = Rsi::new((32, 32));
    {
        let mut state = tester.new_state("ByeThere", &[], 1);

        let mut vec = state.get_icons_vec_mut();
        vec[0] = vec![(DynamicImage::new_rgba8(32, 32), 1.0), (DynamicImage::new_rgba8(32, 32), 1.0), (DynamicImage::new_rgba8(32, 32), 1.0), (DynamicImage::new_rgba8(32, 32), 1.0)];
    }

    tester.new_state("HiThere", &[], 1);

    if !rsi.metadata_equality(&tester) {
        println!("{:?}\n{:?}", rsi, tester);
        panic!("RSI metadata incorrect.");
    }

}
