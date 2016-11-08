extern crate shared;
use shared::test;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::{Image, clear, draw_state};
use std::path::Path;
use piston::input::RenderEvent;

fn main() {
    test::test();

    create_window();
}

fn create_window() {
    let opengl  = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Useless Window!", [640, 640])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl  = GlGraphics::new(opengl);

    //Create the image object and attach a square Rectangle object inside.
    let image   = Image::new();
    //A texture to use with the image
    let texture = Texture::from_path(Path::new("speed.png")).unwrap();

    let drawstate = draw_state::DrawState::default();

    //Main loop
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                //Clear the screen
                clear([0.0, 0.0, 0.0, 1.0], gl);
                //Draw the image with the texture
                image.draw(&texture, &drawstate, c.transform, gl);
            });
        }
    }
}
