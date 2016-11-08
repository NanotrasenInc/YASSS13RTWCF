extern crate shared;
use shared::test;

extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston_window;

use piston_window::{Button, Event, Events, EventLoop, Input, Motion, PistonWindow,
    WindowSettings};
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::{Image, clear, draw_state};
use std::path::Path;
use piston::input::{RenderEvent, Key, UpdateEvent};

fn main() {
    test::test();



    create_game_window();
}

fn create_game_window() {
    let opengl  = OpenGL::V3_2;

    let mut game = GameWindow{locationx : 0.0, locationy : 0.0, actions : UserActions::default()};
    let mut window: PistonWindow = WindowSettings::new("Useless Window!", [640, 640])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl  = GlGraphics::new(opengl);

    //Create the image object and attach a square Rectangle object inside.
    let image   = Image::new();
    //A texture to use with the image
    let texture = Texture::from_path(Path::new("../icons/screenshot.png")).unwrap();

    let drawstate = draw_state::DrawState::default();

    //Main loop
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {

        match e {
            Event::Input(Input::Press(Button::Keyboard(key))) => {
                game.key_press(key);
            }

            Event::Input(Input::Release(Button::Keyboard(key))) => {
                game.key_release(key);
            }

            Event::Render(args) => {
                gl.draw(args.viewport(), |c, gl| {
                    //Clear the screen
                    clear([0.0, 0.0, 0.0, 1.0], gl);
                    //Draw the image with the texture
                    use piston_window::Transformed;
                    let transform = c.transform.trans(game.locationx, game.locationy);

                    image.draw(&texture, &drawstate, transform, gl);
                });
            }

            Event::Update(args) => {
                game.update(args.dt);
            }

            _ => {}
        }
    }
}

//Store info about the game window for rendering, controlled by the server eventually???
pub struct GameWindow {
    //Temporarily using this to decide render location I think
    locationx: f64,
    locationy: f64,
    //Using this to store actions which are eventually going to be communicated to server, for now activate clientside functions
    actions: UserActions
}

impl GameWindow {
/// Processes a key press
    pub fn key_press(&mut self, key: Key) {
        self.handle_key(key, true);
    }

    /// Processes a key release
    pub fn key_release(&mut self, key: Key) {
        self.handle_key(key, false);
    }

    /// Handles a key press or release
    fn handle_key(&mut self, key: Key, pressed: bool) {
        match key {
            Key::Left => self.actions.move_left = pressed,
            Key::Right => self.actions.move_right = pressed,
            Key::Up => self.actions.move_up = pressed,
            Key::Down => self.actions.move_down = pressed,
            _ => ()
        }
    }

    fn update(&mut self, dt: f64) {
        if self.actions.move_left{
            self.locationx += 2.0;}
        if self.actions.move_right{
            self.locationx -= 2.0;}
        if self.actions.move_up{
            self.locationy += 2.0;}
        if self.actions.move_down {
            self.locationy -= 2.0;}
    }

}

//Using this to store actions which are eventually going to be communicated to server, for now activate clientside functions
#[derive(Default)]
pub struct UserActions {
    move_left: bool,
    move_right: bool,
    move_up: bool,
    move_down: bool,
}
