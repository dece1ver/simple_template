mod config;
mod logger;

use eyre::Result;
use tracing::debug;

fn main() -> Result<()> {
    color_eyre::install()?;
    let config = config::load_config()?;
    let _guard = logger::init(&config)?;
    debug!("App initialized.");

    println!("Hello, world!");

    Ok(())
}
