use std::path::{PathBuf, Path};
use shared::entities::components::Component;
use std::collections::HashMap;
use piston_window::*;
use gfx_device_gl::{Resources, Factory};
use shared::assets::{ASSET_MANAGER, Asset};
use nalgebra::Vector2;
use shared::entities::WORLD;
use shared::entities::components::PositionComponent;
use shared::rsi::RsiRef;

#[derive(Debug)]
pub struct RenderableComponent {
    image: PathBuf,
    rsiref: RsiRef
}

impl Component for RenderableComponent {}

impl RenderableComponent {
    pub fn new(image: &Path, rsiref: &RsiRef) -> RenderableComponent {
        RenderableComponent {
            image: image.to_owned(),
            rsiref: rsiref.clone()
        }
    }
}

pub struct Renderer {
    textures: HashMap<(PathBuf, RsiRef), Texture<Resources>>,
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
                for state in rsi.iter_states() {
                    if state.is_empty() {
                        continue;
                    }

                    let stateid = state.to_stateid();
                    for (dir, icons) in state.get_icons_vec().iter().enumerate() {
                        for (frame, &(ref image, _)) in icons.iter().enumerate() {
                            let texture = Texture::from_image(
                                factory,
                                &image.to_rgba().clone(),
                                &TextureSettings::new()
                            ).unwrap();
                            let rsiref = RsiRef::new(&stateid, dir as u8, frame);
                            self.textures.insert((path.clone(), rsiref), texture);
                        }
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
                // TODO: Don't clone() this shit you idiot.
                let texture = self.textures.get(&(renderable.image.clone(), renderable.rsiref.clone())).unwrap();
                image(texture, c.trans(new_coords[(0, 0)].round(), new_coords[(1, 0)].round()).transform, g)
            }
        }
    }
}
