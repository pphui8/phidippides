//! src/database.rs
//! 
//! 

pub mod database {
    use mysql::*;
    use mysql::prelude::*;
    use std::collections::HashMap;

    /// 获取 blog 目录
    pub fn get_index() -> HashMap<String, String> {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 查询
        let res: Vec<(String, String)> = conn.query("select name, descript from blog").unwrap();
        // 转为 HashMap<String, String>
        let mut index_map = HashMap::new();
        for (name, desc) in res {
            index_map.insert(name, desc);
        }
        index_map
    }

    /// 获取特定 blog
    /// 
    /// return: blog 的根url
    pub fn get_blog(blog_name: String) -> String {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 查询
        let mut query = String::from("select article from blog where name=\"");
        query.push_str(&blog_name);
        query.push_str("\"");
        let res: Vec<String> = conn.query(query).unwrap();
        // 返回结果
        res[0].clone()
    }
}

#[cfg(test)]
mod database_tests {
    use super::database::*;

    #[test]
    fn test_index() {
        let res = get_index();
        println!("result from database/myblog/blog:");
        println!("{:?}", res);
    }
}