use slog::Logger;
use logs::LOGGER as HEAD_LOGGER;

pub mod manager;
pub mod asset;

lazy_static! {
    pub static ref LOGGER: Logger = {
        HEAD_LOGGER.new(None)
    };
}


pub use self::asset::Asset;
pub use self::manager::{load_from_dir, get_asset};
