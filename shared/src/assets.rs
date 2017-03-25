use slog::Logger;
use logs::LOGGER as HEAD_LOGGER;
use rsi::Rsi;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::hash_map::Iter as HashMapIter;
use std::path::{Path, PathBuf};
use std::sync::{RwLock, Arc};
use std::io::Result as IoResult;

lazy_static! {
    #[doc(hidden)]
    pub static ref LOGGER: Logger = {
        HEAD_LOGGER.new(None)
    };

    /// The global asset manager all things should use.
    pub static ref ASSET_MANAGER: RwLock<AssetManager> = {
        RwLock::new(AssetManager {assets: HashMap::new(), root: PathBuf::new()})
    };
}

/// An "asset". Assets can be things such as images, sound, binary files, etc...
#[derive(Debug, Clone)]
pub enum Asset {
    /// A simplistic binary file.
    ///
    /// This is basically the category a file falls under if nothing else was found for it.
    Binary(Vec<u8>),

    /// An RSI.
    Rsi(Rsi),
}

impl Asset {
    /// Returns `true` if this asset is an `Asset::Binary`.
    pub fn is_binary(&self) -> bool {
        match self {
            &Asset::Binary(_) => true,
            _ => false,
        }
    }

    /// Reads the "bytes" of this asset.
    /// Some asset types such as RSIs do not have direct byte representations.
    pub fn as_bytes(&self) -> Option<&[u8]> {
        if let Asset::Binary(ref vec) = *self {
            return Some(vec.as_slice());
        }
        None
    }

    /// Returns `true` if this asset is an RSI.
    pub fn is_rsi(&self) -> bool {
        match self {
            &Asset::Rsi(_) => true,
            _ => false,
        }
    }

    /// Returns an RSI if this asset is indeed an RSI.
    pub fn as_rsi(&self) -> Option<&Rsi> {
        match self {
            &Asset::Rsi(ref rsi) => Some(&rsi),
            _ => None,
        }
    }

    /// Recursively ran over files by the `AssetManager` to load assets.
    ///
    /// If this returns `None` on a file, the file is ignored.
    #[doc(hidden)]
    pub fn from_file(path: &Path) -> Option<Self> {
        // TODO: Can we softcode this?

        match File::open(path) {
            Ok(mut file) => {
                let mut buf = Vec::new();
                match file.read_to_end(&mut buf) {
                    Ok(_) => return Some(Asset::Binary(buf)),
                    Err(error) => {
                        error!(LOGGER, "Failed to read from file.";
                            "error" => format!("{:?}", error), "path" => format!("{:?}", path));
                        return None;
                    }
                }
            }
            Err(error) => {
                error!(LOGGER, "Failed to open file.";
                    "error" => format!("{:?}", error), "path" => format!("{:?}", path));
                return None;
            }
        }
        //None
    }

    /// Recursively ran over directories by the `AssetManager` to load assets.
    ///
    /// This is ran before files,
    /// if the second value of the tuple return is `false`,
    /// the contents of the directory are ignored.
    #[doc(hidden)]
    pub fn from_dir(path: &Path) -> (Option<Asset>, bool) {
        if let Some(string) = path.extension().and_then(|x| x.to_str()) {
            return match string {
                "rsi" => {
                    match Rsi::open(path) {
                        Ok(rsi) => (Some(Asset::Rsi(rsi)), false),
                        Err(error) => {
                            error!(LOGGER, "Failed to open RSI.";
                    "error" => format!("{:?}", error), "path" => format!("{:?}", path));
                            (None, false)
                        }
                    }
                }
                _ => (None, true),
            };
        }
        (None, true)
    }
}

// Ah yes a 1000 lines in and I'm already writing shitcode.
// This project is going great.

/// Manages all assets.
pub struct AssetManager {
    assets: HashMap<PathBuf, Arc<Asset>>,

    /// The absolute path to which the assets are relative.
    root: PathBuf,
}

impl AssetManager {
    /// Returns an iterator over all loaded assets.
    pub fn iter<'a>(&'a self) -> AssetIter<'a> {
        AssetIter { iter: self.assets.iter() }
    }
}

/// Get an asset by path relative from the directory assets were loaded from.
///
/// This means that if assets were loaded from directory `/a`,
/// `b/c` would point to `/a/b/c`
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
        } else if entry_type.is_file() {
            if let Some(asset) = Asset::from_file(path) {
                map.insert(relative.to_owned(), Arc::new(asset));
            }
        }
        // TODO: Do we care about symlinks?
        // Probably not but eh.
    }

    Ok(())
}

/// An iterator over all the loaded assets.
pub struct AssetIter<'a> {
    iter: HashMapIter<'a, PathBuf, Arc<Asset>>,
}

impl<'a> Iterator for AssetIter<'a> {
    type Item = (&'a PathBuf, &'a Arc<Asset>);

    fn next(&mut self) -> Option<(&'a PathBuf, &'a Arc<Asset>)> {
        self.iter.next()
    }
}