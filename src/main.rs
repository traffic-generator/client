use std::path::Path;

// use clap::App;

// mod hex;
mod log;
// mod protocol;

use log::Logger;

fn main() {
    let mut log = Logger::new();
    log.log_to_file(Path::new("./logs/log.log"));
    log.info("Start traffic generator client".to_string());
}
