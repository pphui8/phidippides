//! this mod is to read config.toml and prase it 

pub mod config {
    pub enum IPAddr {
        V4(u8, u8, u8, u8),
        // V6(String),  unsupport
    }
    pub struct Server {
        ip: IPAddr,
        port: u64,
        server_name: Option<String>,
    }
    fn read_config() {

    }
}