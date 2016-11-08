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
use graphics::{Image, clear, draw_state, DrawState};
use graphics::rectangle::square;
use std::path::Path;

fn main() {
    let opengl  = OpenGL::V3_2;
    let mut gl  = GlGraphics::new(opengl);
/*    let window  = Window::new(
            opengl,
            WindowSettings::new(
                "Example",
                [600, 400]
            ).exit_on_esc(true));
*/

    let mut window: Window = WindowSettings::new("Useless Window!", [640, 640])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //Create the image object and attach a square Rectangle object inside.
    let image   = Image::new().rect(square(0.0, 0.0, 200.0));
    //A texture to use with the image
    let texture = Texture::from_path(Path::new("speed.png")).unwrap();

    let draw_state = draw_state::DrawState::default();

    //Main loop
    while let Some(e) = window.events() {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                //Clear the screen
                clear([0.0, 0.0, 0.0, 1.0], gl);
                //Draw the image with the texture
                image.draw(&texture, &draw_state, c.transform, gl);
            });
        }
    }
}
