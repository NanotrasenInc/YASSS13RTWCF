extern crate piston_window;
extern crate yasss13rtwcf_shared as shared;
#[macro_use]
extern crate slog;
extern crate slog_term;
#[macro_use]
extern crate lazy_static;
extern crate gfx_device_gl;
extern crate image;
extern crate nalgebra;
extern crate input;
extern crate bytes;
extern crate tokio_io;
extern crate tokio_core;
extern crate futures;

mod logs;
mod rendering;

use piston_window::*;
use logs::LOGGER;
use shared::assets::load_from_dir;
use std::env;
use shared::entities::{WORLD, make_builder};
use self::rendering::{RenderableComponent, Renderer};
use shared::entities::components::PositionComponent;
use shared::entities::components::position::Positional;
use std::path::Path;
use std::collections::HashSet;
use input::{Key, Button, UpdateArgs};
use shared::rsi::{RsiRef, StateId};
use nalgebra::core::Vector2;

fn main() {
    info!(LOGGER, "Starting client"; "version" => env!("CARGO_PKG_VERSION"));

    {
        // Register client components with the ECS world.
        let mut world = WORLD.write().unwrap();
        world.register_component::<RenderableComponent>();
    }

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

    let mut window: PistonWindow =
        WindowSettings::new("YASSS13RTWCF", [640, 480]).exit_on_esc(true).build().unwrap();

    make_builder(&WORLD)
        .with_component(PositionComponent::empty())
        .with_component(RenderableComponent::new(&Path::new("renderingtest.rsi"),
                                                 &RsiRef::new(&StateId::new("toolbox"), 0, 0)));

    make_builder(&WORLD)
        .with_component(PositionComponent::new(Positional::new(Vector2::new(100.0, 100.0), 0)))
        .with_component(RenderableComponent::new(&Path::new("renderingtest.rsi"),
                                                 &RsiRef::new(&StateId::new("Ytoolbox"), 0, 0)));


    let mut renderer = Renderer::new();
    renderer.load_textures(&mut window.factory);

    let mut keys = HashSet::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| renderer.render(c, g));

        e.press(|x| keys.insert(x));
        e.release(|x| keys.remove(&x));

        e.update(|&UpdateArgs { dt: delta }| {
            if keys.contains(&Button::Keyboard(Key::Up)) {
                renderer.camera[(1, 0)] -= 300.0 * delta;
            } else if keys.contains(&Button::Keyboard(Key::Down)) {
                renderer.camera[(1, 0)] += 300.0 * delta;
            }

            if keys.contains(&Button::Keyboard(Key::Left)) {
                renderer.camera[(0, 0)] -= 300.0 * delta;
            } else if keys.contains(&Button::Keyboard(Key::Right)) {
                renderer.camera[(0, 0)] += 300.0 * delta;
            }
        });
    }
}
