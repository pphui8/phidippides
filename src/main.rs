//! crate/main.rs
//! 
//! 程序的启动入口
//! ## exit code definition
//! exit code 1: 程序执行出错，请按照提示修改
//! exit code 0: 程序顺利执行

use std::env;
use phidippides::{command, config};

fn main() {
    let args: Vec<String> = env::args().collect();
    command::commands::deal_args(&args);
    println!("process continue...");
    let _config = config::configs::deal_config();
    println!("{:?}", _config);
    

}