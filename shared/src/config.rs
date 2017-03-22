extern crate toml;

use toml::{ParserError, Parser, Table, Value};
use std::sync::RwLock;
use std::path::Path;
use std::io;
use std::io::Read;
use std::fs::File;

pub struct Config {
    toml: Table,
}

impl Config {
    /// Load the config from the string.
    pub fn load(&mut self, data: &str) -> Result<(), Vec<ParserError>> {
        let mut parser = Parser::new(data);
        if let Some(value) = parser.parse() {
            self.toml = value;
            return Ok(());
        }

        Err(parser.errors)
    }

    /// Load a configuration file.
    pub fn load_file(&mut self, path: &Path) -> Result<(), ConfigError> {
        let mut data = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut data)?;
        self.load(&data)?;
        Ok(())
    }

    /// Access a value on the config file.
    ///
    /// The key used is a period separated path such as "connection.port".
    pub fn get(&self, key: &str) -> Option<Value> {
        let key = String::from(key);
        let mut current = &self.toml;

        for name in key.split('.') {
            if !current.contains_key(name) {
                return None;
            }
            if let Value::Table(ref table) = current[name] {
                current = table;
            } else {
                return Some(current[name].clone());
            }
        }
        None
    }
}

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = {
        // Initialize it to nothing.
        RwLock::new(Config { toml: toml::Table::new() })
    };
}

#[derive(Debug)]
pub enum ConfigError {
    /// An IO error occured.
    Io(io::Error),

    /// The TOML failed to parse.
    ParserError(Vec<ParserError>),
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<Vec<ParserError>> for ConfigError {
    fn from(err: Vec<ParserError>) -> ConfigError {
        ConfigError::ParserError(err)
    }
}
