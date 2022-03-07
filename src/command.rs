//! crate/src/command.rs
//! 
//! 接收程序启动的参数并处理
//! 
//! # Warning!!!
//! # !!!注意，程序运行至此包后不会继续执行

pub mod commands {
    use std::process;

    pub enum Options {
        Help,
        Version,
        Filepath,
        None,
    }

    pub struct Configs {
        version: String,
        file_path: String,
        option: Options,
    }

    /// 处理参数函数
    pub fn deal_args(args: &[String]) {
        let mut res = Configs {
            version: String::from("1.0"),
            file_path: String::from("phidippides/config.toml"),
            option: Options::None,
        };
        if args.len() == 2 {
            match args[1].as_str() {
                "version" => { res.option = Options::Version; }
                "help" => { res.option = Options::Help; }
                "filepath" => { res.option = Options::Filepath; }
                _ => { res.option = Options::Help; }
            }
            match res.option {
                Options::Help => { print_help(&res); }
                Options::Version => { println!("phidippides version: phidippides/{}", res.version); }
                Options::Filepath => { println!("{}", res.file_path) }
                _ => { print_help(&res); }
            }
            process::exit(0);
        }
    }

    /// 输出可选参数
    fn print_help(configs: &Configs) {
        println!("phidippides version: phidippides/{}", configs.version);
        println!("Usage: phid [option]");
        println!();
        println!("Options:");
        println!("help      : this help");
        println!("version   : phidippides version");
        println!("filepath  : configuration file path")
    }

}