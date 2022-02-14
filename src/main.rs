mod config;
mod command;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    command::commands::deal_args(&args);
    println!("process continue...");
    // config.clone();
}