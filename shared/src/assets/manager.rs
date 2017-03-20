//! Manages game assets.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use assets::{Asset, LOGGER};

lazy_static! {
    pub static ref ASSET_MANAGER: Mutex<AssetManager> = {
        Mutex::new(AssetManager {assets: HashMap::new(), root: PathBuf::new()})
    };
}

pub struct AssetManager {
    assets: HashMap<PathBuf, Asset>,

    /// The absolute path to which the assets are relative.
    root: PathBuf
}

pub fn get_asset<P: AsRef<Path>>(path: P) -> Option<Asset> {
    let path = path.as_ref();
    let manager = ASSET_MANAGER.lock().unwrap();

    if let Some(asset) = manager.assets.get(path) {
        Some(asset.clone())
    } else {
        None
    }
}

// TODO: Async here, error handling. Unwrap my shit up!
/// Loads a directory into the global asset manager.
///
/// The path used will be the "root" for the loaded files, and must be absolute.
/// # Panics.
/// Panics if the path isn't absolute.
/// Or if, you know, any I/O error occurs.
/// I promise I'll remove the `unwrap()`s guys.
pub fn load_from_dir<P: AsRef<Path>>(path: P) {
    let path = path.as_ref();
    let mut new_assets = HashMap::new();
    let new_root = path.to_owned();

    if !new_root.is_absolute() {
        panic!("Path provided is not absolute.")
    }

    _load_dir(path, &new_root, &mut new_assets);

    let mut manager = ASSET_MANAGER.lock().unwrap();
    manager.assets = new_assets;
    manager.root = new_root;

    info!(LOGGER, "Assets loaded from directory"; "path" => format!("{:?}", path));
}

/// Recursive function to load all files and subfiles in a directory.
fn _load_dir(path: &Path, root: &Path, map: &mut HashMap<PathBuf, Asset>) {
    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = &entry.path();
        let entry_type = entry.file_type().unwrap();
        let relative = path.strip_prefix(root).unwrap();

        if entry_type.is_dir() {
            let (asset, cont) = Asset::from_dir(path);
            if let Some(asset) = asset {
                map.insert(relative.to_owned(), asset);
            }
            if cont {
                _load_dir(path, root, map);
            }
        }

        else if entry_type.is_file() {
            if let Some(asset) = Asset::from_file(path) {
                map.insert(relative.to_owned(), asset);
            }
        }
        // TODO: Do we care about symlinks?
        // Probably not but eh.
    }
}
