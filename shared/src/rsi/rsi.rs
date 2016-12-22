use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path};
use super::helpers::full_state_name;
use super::{RsiError, RsiSelectors, State, MAXIMUM_RSI_VERSION, MINIMUM_RSI_VERSION};
use rustc_serialize::json::Json;

/// Represents an RSI.
#[allow(dead_code)]
pub struct Rsi {
    /// The dimensions of the RSI's images.
    size: (u32, u32),

    /// The states!
    states: HashMap<String, State>
}

#[allow(dead_code)]
impl Rsi {
    /// Gets a state by name, without selectors.
    pub fn get(&self, name: &str) -> Option<&State> {
        self.states.get(&name.to_string())
    }

    /// Gets a state by name, with selectors.
    pub fn get_select(&self, name: &str, select: &[RsiSelectors]) -> Option<&State> {
        self.get(&full_state_name(name, select))
    }

    /// Gets a mutable state by name, without selectors.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut State> {
        self.states.get_mut(&name.to_string())
    }

    /// Gets a mutable state by name, with selectors.
    pub fn get_select_mut(&mut self, name: &str, select: &[RsiSelectors]) -> Option<&mut State> {
        self.get_mut(&full_state_name(name, select))
    }

    /// Makes a new state and adds it to this RSI.
    ///
    /// If a state already exists with this name and selectors, it will be overriden.
    pub fn new_state(&mut self, name: &str, select: &[RsiSelectors], directions: u8) -> &mut State {
        let state = State::new(name, select, self.size, directions);
        self.add_state(state);
        self.get_select_mut(name, select).unwrap()
    }

    /// Adds an existing state to this RSI, overriding any with the same identifying values.
    pub fn add_state(&mut self, state: State) {
        self.states.insert(state.get_full_name().to_string(), state);
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }
}

#[allow(dead_code)]
impl Rsi {
    /// Opens an RSI from the file system.
    ///
    /// TODO: Make this return a proper error.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Rsi, RsiError> {
        let path = path.as_ref();

        // Open and read file to meta_content, return an Err if anything failed.
        let mut meta_content = String::new();
        match File::open(path.join("meta.json")) {
            Ok(mut f) => {
                if let Err(error) = f.read_to_string(&mut meta_content) {
                    return Err(RsiError::IO(error));
                }
            },
            Err(error) => return Err(RsiError::IO(error))
        };

        let json = match Json::from_str(&meta_content) {
            Ok(x) => if let Json::Object(a) = x {
                        a
                     } else {
                        return Err(RsiError::Metadata("Not a root object".to_string()));
                     },
            Err(error) => return Err(RsiError::Json(error))
        };

        match json.get("version") {
            Some(ref x) => match **x {
                Json::U64(version) => if MAXIMUM_RSI_VERSION < version || version < MINIMUM_RSI_VERSION {
                    return Err(RsiError::Version);
                },
                _ => return Err(RsiError::Metadata("Version not a number.".to_string()))
            },
            None => return Err(RsiError::Metadata("Version not included.".to_string()))
        }

        // Surely there's a better way for this, right?
        let mut size: (u32, u32) = (0, 0);
        match json.get("size") {
            Some(ref x) => match **x {
                Json::Object(ref o) => {
                    match o.get("x") {
                        Some(ref x) => match **x {
                            Json::U64(ref x) => size.0 = *x as u32,
                            _ => return Err(RsiError::Metadata("size: x not a number.".to_string()))
                        },
                        None => return Err(RsiError::Metadata("Size: x not included.".to_string()))
                    }

                    match o.get("y") {
                        Some(ref x) => match **x {
                            Json::U64(ref y) => size.1 = *y as u32,
                            _ => return Err(RsiError::Metadata("Size: y not a number.".to_string()))
                        },
                        None => return Err(RsiError::Metadata("Size: x not included.".to_string()))
                    }
                },
                _ => return Err(RsiError::Metadata("Size not an object.".to_string()))
            },
            None => return Err(RsiError::Metadata("Size not included.".to_string()))
        }

        let states = match json.get("states") {
            Some(ref x) => match **x {
                Json::Array(ref array) => {
                    array
                },
                _ => return Err(RsiError::Metadata("States not an array.".to_string()))
            },
            None => return Err(RsiError::Metadata("states not included.".to_string()))
        };

        let mut rsi = Rsi {
            size: size,
            states: HashMap::with_capacity(states.len())
        };

        for json in states {
            match *json {
                Json::Object(ref o) => rsi.add_state(State::from_json(&o, path, size)?),
                _ => return Err(RsiError::Metadata("State not an object.".to_string()))
            };
        }

        Ok(rsi)
    }
}
