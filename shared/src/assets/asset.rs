use rsi::Rsi;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use assets::LOGGER;

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

    pub fn is_rsi(&self) -> bool {
        match self {
            &Asset::Rsi(_) => true,
            _ => false,
        }
    }

    pub fn as_rsi(&self) -> Option<&Rsi> {
        match self {
            &Asset::Rsi(ref rsi) => Some(&rsi),
            _ => None,
        }
    }

    /// Recursively ran over files by the `AssetManager` to load assets.
    ///
    /// If this returns `None` on a file, the file is ignored.
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
