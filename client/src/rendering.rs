use std::path::PathBuf;
use shared::entities::components::Component;
use std::collections::HashMap;
use piston_window::*;
use gfx_device_gl::{Resources, Factory};
use shared::assets::{ASSET_MANAGER, Asset};
use nalgebra::Vector2;
use shared::entities::WORLD;
use shared::entities::components::PositionComponent;

pub struct RenderableComponent {
    pub image: PathBuf
}

impl Component for RenderableComponent {}


pub struct Renderer {
    textures: HashMap<PathBuf, Texture<Resources>>,
    pub camera: Vector2<f64>
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            textures: HashMap::new(),
            camera: Vector2::new(0.0, 0.0)
        }
    }

    pub fn load_textures(&mut self, factory: &mut Factory) {
        let manager = ASSET_MANAGER.read().unwrap();
        for (path, asset) in manager.iter() {
            if let Asset::Rsi(ref rsi) = **asset {
                // TODO: States and everything else.
                if let Some(state) = rsi.iter_states().next() {
                    if let Some(image) = state.get_icon(0, 0) {
                        let texture = Texture::from_image(
                            factory,
                            &image.to_rgba().clone(),
                            &TextureSettings::new()
                        ).unwrap();
                        self.textures.insert(path.clone(), texture);
                    }
                }
            }
        }
    }

    pub fn render(&self, c: Context, g: &mut G2d) {
        clear([0.0; 4], g);
        let world = WORLD.read().unwrap();
        for (id, component) in world.iter_components::<RenderableComponent>() {
            if let Some(position) = world.get_component::<PositionComponent>(id) {
                let renderable = component.read().unwrap();
                let pos = position.read().unwrap().get_position();
                let new_coords = pos.coordinates - self.camera;
                image(self.textures.get(&renderable.image).unwrap(), c.trans(new_coords[(0, 0)].round(), new_coords[(1, 0)].round()).transform, g)
            }
        }
    }
}
