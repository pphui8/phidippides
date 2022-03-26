//! crate/main.rs
//! 
//! 程序的启动入口
//! ## exit code definition
//! exit code 1: 程序执行出错，请按照提示修改
//! exit code 0: 程序顺利执行

use std::env;
use phidippides::{command, config, server};

fn main() {
    // 获取 args
    let args: Vec<String> = env::args().collect();
    // 传入 command包处理
    command::commands::deal_args(&args);
    println!("process continue...");
    // 读取config
    let config = config::configs::deal_config();
    println!("{:?}", config);
    
    // 开启服务
    server::server::server_start().unwrap();
}