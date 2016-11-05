extern crate toml;

use std::sync::Mutex;
use std::path::Path;
use std::io::Read;
use std::fs::File;

pub struct Config {
    pub toml: toml::Table
}

impl Config {
    // Load the config from the string.
    // TODO: Improve error handling on this, make it return a Result.
    pub fn load(&mut self, data: &str) {
        let mut parser = toml::Parser::new(data);
        match parser.parse() {
            Some(value) => {
                self.toml = value;
            }
            None => {
                println!("Hit parser erros trying to load a config! {:?}", parser.errors);
            }
        }
    }

    // TODO: error handling.
    pub fn load_file(&mut self, path: &Path) {
        let mut data = String::new();
        let mut file = File::open(path).expect("Unable to open config fiile.");
        file.read_to_string(&mut data).expect("Unable to read config file.");
        self.load(&*data);
    }

    // Allows you to access a key in the config with a single string.
    // For example: "connection.port"
    // TODO: Error handling. As usual.
    pub fn get(&self, key: &'static str) -> Option<toml::Value> {
        let key = String::from(key);
        let mut current = &self.toml;

        for name in key.split('.') {
            if !current.contains_key(name) {
                return None;
            }
            if let toml::Value::Table(ref table) = current[name] {
                current = table;
            } else {
                return Some(current[name].clone());
            }
        }
        None
    }

}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = {
        // Initialize it to nothing.
        Mutex::new(Config { toml: toml::Table::new() })
    };
}
