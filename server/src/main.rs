#[macro_use]
extern crate lazy_static;
extern crate yasss13rtwcf_shared as shared;
extern crate toml;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate bytes;
extern crate tokio_io;
extern crate tokio_core;
extern crate futures;
extern crate byteorder;

mod net;
mod logs;

use shared::config;
use shared::assets::load_from_dir;
use std::env;
use logs::LOGGER;


fn main() {
    info!(LOGGER, "Starting server"; "version" => env!("CARGO_PKG_VERSION"));

    // Asset dir is next to the executable, under "data".
    let mut asset_dir = env::current_exe().expect("Unable to find executable path.");
    // Go by data dir in the main project right now.
    // TODO: Fix this for packaged release builds or something.
    asset_dir.pop();
    asset_dir.pop();
    asset_dir.pop();
    asset_dir.push("data");

    info!(LOGGER, "Loading asset directory"; "directory" => format!("{:?}", asset_dir));
    load_from_dir(asset_dir).expect("Failed to load assets.");

    let mut cfg = config::CONFIG.write().unwrap();
    // TODO: Release builds.
    let mut config_path = env::current_exe().expect("Unable to find executable path.");
    config_path.pop();
    config_path.pop();
    config_path.pop();
    config_path.push("server");
    config_path.push("config");
    config_path.push("config.toml");
    cfg.load_file(&config_path).unwrap();

    let port = cfg.get("connection.port").and_then(|x| x.as_integer()).expect("Unable to find port inside configuration file.");

    net::run(port as u16);
}
