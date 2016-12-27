#[macro_use]
extern crate lazy_static;
extern crate shared;
extern crate toml;
#[macro_use]
extern crate slog;
extern crate slog_term;

use shared::config;
use std::path::Path;

mod logs;

use logs::LOGGER;

// uuuuh....
#[allow(dead_code)]
fn main() {
    info!(LOGGER, "Starting server"; "version" => env!("CARGO_PKG_VERSION"));

    let mut cfg = config::CONFIG.lock().unwrap();
    cfg.load_file(Path::new("config/config.toml")).unwrap();

    info!(LOGGER, "Port is {}.", cfg.get("connection.port").expect("Unable to find port inside configuration file."));
}
