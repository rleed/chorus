include!("macros.rs");

pub mod config;
pub mod error;
pub mod globals;
pub mod store;
pub mod types;

use crate::config::Config;
use crate::error::Error;
use crate::globals::GLOBALS;
use crate::store::Store;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    // Get args (config path)
    let mut args = env::args();
    if args.len() <= 1 {
        panic!("USAGE: chorus <config_path>");
    }
    let _ = args.next(); // ignore program name
    let config_path = args.next().unwrap();

    // Read config file
    let mut file = OpenOptions::new().read(true).open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = ron::from_str(&contents)?;
    log::debug!("Loaded config file.");

    // Setup store
    let store = Store::new(&config.data_directory)?;
    let _ = GLOBALS.store.set(store);

    // Bind listener to port
    let _listener = TcpListener::bind((&*config.ip_address, config.port)).await?;
    log::info!("Running on {}:{}", config.ip_address, config.port);

    // Store config into GLOBALS
    *GLOBALS.config.write().await = config;

    log::error!("No main yet.");

    Ok(())
}
