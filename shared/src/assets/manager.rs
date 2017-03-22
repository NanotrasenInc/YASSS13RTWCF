//! Manages game assets.

use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use assets::{Asset, LOGGER};
use std::sync::Arc;
use std::io::Result as IoResult;

// Ah yes a 1000 lines in and I'm already writing shitcode.
// This project is going great.

lazy_static! {
    pub static ref ASSET_MANAGER: RwLock<AssetManager> = {
        RwLock::new(AssetManager {assets: HashMap::new(), root: PathBuf::new()})
    };
}

pub struct AssetManager {
    assets: HashMap<PathBuf, Arc<Asset>>,

    /// The absolute path to which the assets are relative.
    root: PathBuf
}

impl AssetManager {
    pub fn iter<'a>(&'a self) -> AssetIter<'a> {
        AssetIter {
            iter: self.assets.iter()
        }
    }
}

pub fn get_asset<P: AsRef<Path>>(path: P) -> Option<Arc<Asset>> {
    let path = path.as_ref();
    let manager = ASSET_MANAGER.read().unwrap();

    manager.assets.get(path).map(|x| x.clone())
}

// TODO: Some sort of async to speed this up? Long ways off.
/// Loads a directory into the global asset manager.
///
/// The path used will be the "root" for the loaded files, and must be absolute.
/// # Panics.
/// Panics if the path isn't absolute.
pub fn load_from_dir<P: AsRef<Path>>(path: P) -> IoResult<()> {
    let path = path.as_ref();
    let mut new_assets = HashMap::new();
    let new_root = path.to_owned();

    // TODO: This is dumb, past me.
    if !new_root.is_absolute() {
        panic!("Path provided is not absolute.")
    }

    _load_dir(path, &new_root, &mut new_assets)?;

    let mut manager = ASSET_MANAGER.write().unwrap();
    manager.assets = new_assets;
    manager.root = new_root;

    info!(LOGGER, "Assets loaded from directory"; "path" => format!("{:?}", path));

    Ok(())
}

/// Recursive function to load all files and subfiles in a directory.
fn _load_dir(path: &Path, root: &Path, map: &mut HashMap<PathBuf, Arc<Asset>>) -> IoResult<()> {
    for entry in path.read_dir()? {
        let entry = entry?;
        let path = &entry.path();
        let entry_type = entry.file_type()?;
        let relative = path.strip_prefix(root).unwrap();

        if entry_type.is_dir() {
            let (asset, cont) = Asset::from_dir(path);
            if let Some(asset) = asset {
                map.insert(relative.to_owned(), Arc::new(asset));
            }
            if cont {
                _load_dir(path, root, map)?;
            }
        }

        else if entry_type.is_file() {
            if let Some(asset) = Asset::from_file(path) {
                map.insert(relative.to_owned(), Arc::new(asset));
            }
        }
        // TODO: Do we care about symlinks?
        // Probably not but eh.
    }

    Ok(())
}

pub struct AssetIter<'a> {
    iter: HashMapIter<'a, PathBuf, Arc<Asset>>
}

impl<'a> Iterator for AssetIter<'a> {
    type Item = (&'a PathBuf, &'a Arc<Asset>);

    fn next(&mut self) -> Option<(&'a PathBuf, &'a Arc<Asset>)> {
        self.iter.next()
    }
}
