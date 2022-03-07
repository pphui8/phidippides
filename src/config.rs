//! crate/src/config.rs
//! 
//! 配置文件处理包
//! 配置文件路径: /config.toml

pub mod configs {
    use std::fs::{OpenOptions, File};
    use std::io::Read;
    use serde::Deserialize;
    use std::process;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    pub struct Config {
        server: Server,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    pub struct Server {
        ip: Option<String>,
        port: Option<u64>,
        root: Option<String>,
    }
    pub fn deal_config() -> Config {
        let mut file = OpenOptions::new()
            .read(true)
            .open("config.toml").unwrap();
        read_file_to_toml(&mut file)
    }

    /// 参数文件并读取
    /// # Error
    /// - failed to open the file
    /// 配置文件输入错误，检查文件位置
    /// - fail to trans file into toml file
    /// 配置文件格式错误
    fn read_file_to_toml(file: &mut File) -> Config {
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap_or_else( |_| {
            eprintln!("fail to read the file");
            process::exit(0);
        });
        let res: Config = toml::from_str(buf.trim()).unwrap_or_else(|_| {
            eprintln!("fail to trans file into toml file");
            eprintln!("please check out the formal of config file");
            process::exit(0);
        });
        // println!("{:?}", res);
        res
    }
}