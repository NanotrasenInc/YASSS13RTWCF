#[macro_use]
extern crate lazy_static;
extern crate shared;
extern crate toml;
#[macro_use]
extern crate slog;
extern crate slog_term;

mod logs;

use shared::config;
use shared::assets::{load_from_dir};
use std::path::Path;
use std::env;
use logs::LOGGER;


// uuuuh....
#[allow(dead_code)]
fn main() {
    info!(LOGGER, "Starting server"; "version" => env!("CARGO_PKG_VERSION"));

    // Asset dir is next to the executable, under "data".
    let mut asset_dir = env::current_exe().expect("Unable to find executable path.");
    asset_dir.pop();
    asset_dir.push("data");

    info!(LOGGER, "Loading asset directory"; "directory" => format!("{:?}", asset_dir));
    load_from_dir(asset_dir);

    let mut cfg = config::CONFIG.write().unwrap();
    cfg.load_file(Path::new("config/config.toml")).unwrap();

    info!(LOGGER, "Port is {}.", cfg.get("connection.port").expect("Unable to find port inside configuration file."));
}
