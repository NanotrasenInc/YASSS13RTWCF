use slog::{Logger, DrainExt};
use slog_term::streamer;

lazy_static! {
    pub static ref LOGGER: Logger = {
        let drain = streamer().build().fuse();

        Logger::root(drain, None)
    };
}
