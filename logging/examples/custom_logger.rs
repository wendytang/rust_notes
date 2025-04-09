use log::{error, info, warn};
use env_logger::Env;

fn main() {
    // Initialize with custom environment configuration
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Application starting up");
    warn!("This is a warning message");
    error!("This is an error message");
}