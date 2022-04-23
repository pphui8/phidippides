pub mod status {
    use std::process::Command;
    use rocket::serde::{Serialize, Deserialize};
    use rocket::serde::json::serde_json;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct RamStatus {
        pub total: String,
        pub used: String,
        pub free: String,
    }

    pub fn get_ram() -> String {
        let result = deal_ram();
        if result == String::from("failed") {
            return "failed".to_string()
        }
        let result: Vec<&str> = result.split(' ').collect();
        let mut useful = Vec::new();
        for i in &result {
            if i.len() > 0 {
                useful.push(i);
            }
        }
        let res: RamStatus = RamStatus{
            total: useful[6].to_string(),
            used: useful[7].to_string(),
            free: useful[8].to_string(),
        };
        serde_json::to_string(&res).unwrap()
    }

    pub fn deal_ram() -> String {
        let output = Command::new("free")
        .arg("-m")
        .output()
        .expect("failed to execute process");

        let result = match output.status.success() {
            true => {
                String::from_utf8_lossy(&output.stdout).to_string()
            },
            false => String::from("failed"),
        };
        result
    }
}

#[cfg(test)]
mod sys_status_tests {
    use super::status::*;

    #[test]
    fn test_deal_ram() {
        let res = deal_ram();
        println!("result from sys_status/deal_ram:");
        println!("{:?}", res);
    }

    #[test]
    fn test_get_ram() {
        let res = get_ram();
        println!("{}", res);
    }
}