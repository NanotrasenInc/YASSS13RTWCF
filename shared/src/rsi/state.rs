use image::{GenericImage, DynamicImage, open as image_open};
use rustc_serialize::json::{Json, Object};
use std::path::Path;
use rsi::{RsiFlags, RsiSelectors, full_state_name, RsiError};
use std::fmt;

#[allow(dead_code)]
#[derive(Clone)]
pub struct State {
    name: String,
    full_name: String,

    size: (u32, u32),
    directions: u8,

    selectors: Vec<RsiSelectors>,
    flags: Vec<RsiFlags>,

    icons: Vec<Vec<(DynamicImage, f32)>>
}

impl State {
    /// Default constructor for states.
    pub fn new(name: &str, select: &[RsiSelectors], size: (u32, u32), directions: u8) -> State {
        let mut state = State {
            name: name.to_string(),
            full_name: full_state_name(name, select),

            size: size,
            directions: directions,

            selectors: select.to_vec(),
            flags: Vec::new(),

            icons: Vec::with_capacity(directions as usize)
        };

        for _ in 0..directions {
            state.icons.push(Vec::new());
        }
        state
    }

    pub fn from_json(json: &Object, context: &Path, size: (u32, u32)) -> Result<State, RsiError> {
        let name = match json.get("name") {
            Some(&Json::String(ref name)) => name,
            _ => return Err(RsiError::Metadata("Name not string.".to_string()))
        };

        // TODO: Implement this when we actually have selectors
        let selectors = Vec::new();

        let directions = match json.get("directions") {
            Some(&Json::U64(d)) => d as u8,
            _ => return Err(RsiError::Metadata(format!("Directions not integer: {:?}", json)))
        };

        let mut state = State::new(&name, &selectors, size, directions);
        state.flags = Vec::new();
        state.icons = Vec::with_capacity(directions as usize);

        // Oh god.
        // Please tell me there's a better way.
        let delays: Vec<Vec<f32>> = match json.get("delays") {
            Some(&Json::Array(ref array)) => {
                let mut delays = Vec::with_capacity(directions as usize);
                for direction in 0..directions {
                    match array.get(direction as usize) {
                        Some(&Json::Array(ref array)) => {
                            let mut vec = Vec::with_capacity(array.len());
                            for item in array {
                                match *item {
                                    Json::F64(delay) => vec.push(delay as f32),
                                    _ => return Err(RsiError::Metadata("Delay not float.".to_string()))
                                }
                            }
                            delays.push(vec);
                        },
                        Some(_) => return Err(RsiError::Metadata("Sub array of delays not an array.".to_string())),
                        None => return Err(RsiError::Metadata("Too little directions".to_string()))
                    }
                }
                delays
            },
            Some(_) => return Err(RsiError::Metadata("Invalid states list.".to_string())),

            // If we have no delays specified default to 0 everything.
            None => {
                let mut delays = Vec::with_capacity(directions as usize);
                for _ in 0..directions {
                    delays.push(vec![0.0]);
                }
                delays
            }
        };

        // Open the image of our state.
        let imagepath = context.join(state.full_name.clone() + ".png");
        let mut image = image_open(imagepath)?;

        let sheetdimensions = (image.dimensions().0 / size.0, image.dimensions().1 / size.1);
        let mut counter = 0;
        for direction in delays {
            let mut icons = Vec::with_capacity(direction.len());
            // Now comes the fun part.
            // Cut the image and stuff!
            for delay in direction {
                let cropped = image.crop(
                    counter % sheetdimensions.0 * size.0,
                    counter / sheetdimensions.1 * size.1,
                    size.0, size.1);

                icons.push((cropped, delay));
                counter += 1;
            }
            state.icons.push(icons);
        }

        Ok(state)
    }
}

impl State {
    /// The name of the state.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// The *full* name of the state.
    ///
    /// The full name of a state is the same as the one used on disk as spritesheet filenames.
    pub fn get_full_name(&self) -> &str {
        &self.full_name
    }

    /// Returns a tuple representing the dimensions of the individual icons of this state.
    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Returns the amount of directions in this state.
    pub fn get_directions(&self) -> u8 {
        self.directions
    }

    /// Returns the selectors of this state.
    pub fn get_selectors(&self) -> &[RsiSelectors] {
        &self.selectors
    }

    /// Returns a vector of the flags.
    ///
    /// This is a vector instead of slice to provide consistency with `get_flags_mut()`.
    pub fn get_flags(&self) -> &Vec<RsiFlags> {
        &self.flags
    }

    /// Returns a mutable vector of the flags.
    pub fn get_flags_mut(&mut self) -> &mut Vec<RsiFlags> {
        &mut self.flags
    }

    /// Returns an icon.
    ///
    /// If the direction or index are too large, returns `None`.
    pub fn get_icon(&self, direction: u8, index: usize) -> Option<&DynamicImage> {
        if let Some(dirvec) = self.icons.get(direction as usize) {
            if let Some(icontuple) = dirvec.get(index) {
                return Some(&icontuple.0);
            }
        }
        None
    }

    pub fn get_delay(&self, direction: u8, index: usize) -> Option<f32> {
        if let Some(dirvec) = self.icons.get(direction as usize) {
            if let Some(icontuple) = dirvec.get(index) {
                return Some(icontuple.1);
            }
        }
        None
    }

    pub fn get_icons_vec(&self) -> &Vec<Vec<(DynamicImage, f32)>> {
        &self.icons
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "State {{ full name: {}, size: {:?}, dir: {}, flags: {:?}}}",
               self.full_name, self.size, self.directions, self.flags)
    }
}
