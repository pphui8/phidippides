//! crate/main.rs
//! 
//! 程序的启动入口

mod config;
mod command;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    command::commands::deal_args(&args);
    println!("process continue...");
    let _config = config::configs::deal_config();
}