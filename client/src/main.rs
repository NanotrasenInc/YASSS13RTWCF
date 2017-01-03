extern crate piston_window;
extern crate shared;
#[macro_use]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate lazy_static;

pub mod logs;

use piston_window::*;
use logs::LOGGER;
use shared::assets::load_from_dir;
use std::env;

#[allow(dead_code)]
fn main() {
    info!(LOGGER, "Starting client"; "version" => env!("CARGO_PKG_VERSION"));

    // Asset dir is next to the executable, under "data".
    let mut asset_dir = env::current_exe().expect("Unable to find executable path.");
    asset_dir.pop();
    asset_dir.push("data");

    info!(LOGGER, "Loading asset diretory"; "directory" => format!("{:?}", asset_dir));
    load_from_dir(asset_dir);

    let mut window: PistonWindow =
    WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
