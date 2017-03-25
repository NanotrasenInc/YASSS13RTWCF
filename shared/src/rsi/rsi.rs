use std::collections::{HashMap, hash_map};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use super::*;
use rustc_serialize::json::Json;

/// Represents an RSI.
#[derive(Debug, Clone)]
pub struct Rsi {
    /// The dimensions of the RSI's images.
    size: (u32, u32),

    /// The states!
    states: HashMap<StateId, State>,
}

impl Rsi {
    /// Gets a state by name, without selectors.
    pub fn get(&self, name: &str) -> Option<&State> {
        self.states.get(&StateId::new(name))
    }

    /// Gets a state by name, with selectors.
    pub fn get_select(&self, name: &str, select: &[RsiSelectors]) -> Option<&State> {
        self.states.get(&StateId::with_select(name, select))
    }

    /// Gets a state by `StateId`
    pub fn get_stateid(&self, id: &StateId) -> Option<&State> {
        self.states.get(id)
    }

    /// Gets a mutable state by name, without selectors.
    pub fn get_mut(&mut self, name: &str) -> Option<&mut State> {
        self.states.get_mut(&StateId::new(name))
    }

    /// Gets a mutable state by name, with selectors.
    pub fn get_select_mut(&mut self, name: &str, select: &[RsiSelectors]) -> Option<&mut State> {
        self.states.get_mut(&StateId::with_select(name, select))
    }

    /// Gets a mutable state by `StateId`
    pub fn get_stateid_mut(&mut self, id: &StateId) -> Option<&State> {
        self.states.get(id)
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
        self.states.insert(state.to_stateid(), state);
    }

    pub fn get_size(&self) -> (u32, u32) {
        self.size
    }

    /// Returns an iterator over the states of this RSI in an arbitrary order.
    pub fn iter_states<'a>(&'a self) -> States<'a> {
        States { iter: self.states.values() }
    }

    /// Checks whether two RSIs have equal metadata.
    /// This does **not** check equality of the images themselves!
    ///
    /// RSIs do not derive `Eq` or `PartialEq`,
    /// due to the high overhead of checking `DynamicImage` equality.
    pub fn metadata_equality(&self, other: &Rsi) -> bool {
        if self.get_size() != other.get_size() {
            return false;
        }

        for state in self.iter_states() {
            match other.get(state.get_full_name()) {
                Some(other_state) => {
                    if !state.metadata_equality(other_state) {
                        return false;
                    }
                }
                _ => return false,
            };
        }

        true
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
        File::open(path.join("meta.json"))?.read_to_string(&mut meta_content)?;

        let json = match Json::from_str(&meta_content)? {
            Json::Object(a) => a,
            _ => return Err(RsiError::Metadata("Not a root object".to_string())),
        };

        match json.get("version") {
            Some(&Json::U64(version)) => {
                if MAXIMUM_RSI_VERSION < version || version < MINIMUM_RSI_VERSION {
                    return Err(RsiError::Version);
                }
            }
            _ => return Err(RsiError::Metadata("Version not a number.".to_string())),
        };

        let size: (u32, u32) = match json.get("size") {
            Some(&Json::Object(ref o)) => {
                (match o.get("x") {
                     Some(&Json::U64(x)) => x as u32,
                     _ => return Err(RsiError::Metadata("Size: x not included.".to_string())),
                 },

                 match o.get("y") {
                     Some(&Json::U64(y)) => y as u32,
                     _ => return Err(RsiError::Metadata("Size: y not included.".to_string())),
                 })
            }
            _ => return Err(RsiError::Metadata("Size not an object.".to_string())),
        };

        let states = match json.get("states") {
            Some(&Json::Array(ref array)) => array,
            _ => return Err(RsiError::Metadata("States not an array.".to_string())),
        };

        let mut rsi = Rsi {
            size: size,
            states: HashMap::with_capacity(states.len()),
        };

        for json in states {
            match *json {
                Json::Object(ref o) => rsi.add_state(State::from_json(&o, path, size)?),
                _ => return Err(RsiError::Metadata("State not an object.".to_string())),
            };
        }

        Ok(rsi)
    }

    /// Returns a new RSI with a set pair of dimensions.
    pub fn new(size: (u32, u32)) -> Rsi {
        Rsi {
            size: size,
            states: HashMap::new(),
        }
    }
}

/// An iterator over all the states in an RSI.
pub struct States<'a> {
    iter: hash_map::Values<'a, StateId, State>,
}

impl<'a> Iterator for States<'a> {
    type Item = &'a State;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// An identifier for a state in an RSI. Contains name and selectors.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StateId {
    pub name: String,
    pub select: Vec<RsiSelectors>,
}

impl StateId {
    /// Create a new StateId with name, but no selectors.
    pub fn new(name: &str) -> StateId {
        StateId {
            name: name.to_owned(),
            select: Vec::new(),
        }
    }

    /// Create a new StateId with name and selectors.
    pub fn with_select(name: &str, select: &[RsiSelectors]) -> StateId {
        StateId {
            name: name.to_owned(),
            select: select.to_owned(),
        }
    }

    /// Returns the "full name" of this state.
    ///
    /// The full name is the name and selectors of a state combined into one string,
    /// for storing on disk. See the RSI spec for more info.
    pub fn to_full_name(&self) -> String {
        full_state_name(&self.name, &self.select)
    }
}

/// Represents a "position" inside an RSI.
///
/// It can be seen as an identifier for all individual icons stored in an RSI.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct RsiRef {
    pub state: StateId,
    pub dir: u8,
    pub frame: usize,
}

impl RsiRef {
    pub fn new(state: &StateId, dir: u8, frame: usize) -> RsiRef {
        RsiRef {
            state: state.clone(),
            dir: dir,
            frame: frame,
        }
    }
}
