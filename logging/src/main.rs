use log::{info, trace, warn};

fn main() {
    env_logger::init();

    info!("starting up");
    warn!("Oops! Something went wrong");
    trace!("Here is a {} complicated", "somewhat");
}