mod config;
mod rpc_client;
mod wallet;
mod scripts;
mod utils;
mod cli;

use wallet::XpubManager;

fn main() {
    // Initialize logger
    env_logger::init();
    log::info!("Starting BoB");

    // Create an XpubManager instance to handle xpub storage
    let mut manager = XpubManager::new("xpub_storage.json");

    // Start the CLI handler
    cli::run(&mut manager);

    log::info!("BoB is ready to interact with GetBlock!");
}
