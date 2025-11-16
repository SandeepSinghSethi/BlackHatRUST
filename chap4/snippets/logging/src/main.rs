use log;
use env_logger;

fn main() {
    env_logger::init();
    //must set RUST_LOG=info/debug/warn/trace .......
    // for libraries using log is sufficient , but for executables env_logger::init() must need to be done.
    println!("Hello, world!");
    log::info!("This is informational");
    log::trace!("This is trace log");
    log::warn!("This is warning log");
    log::debug!("This is debug log");
    log::error!("This is an error log");
}
