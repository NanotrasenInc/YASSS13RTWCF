use slog::{Logger, DrainExt, Discard};
use slog_term::streamer;

lazy_static! {
    /// Global logging object for the shared crate.
    pub static ref LOGGER: Logger = {
        if cfg!(not(test)) {
            let drain = streamer().build().fuse();

            Logger::root(drain, None)
        } else {
            Logger::root(Discard, None)
        }
    };
}
