//! src/database.rs
//! 
//! 

pub mod database {
    use mysql::*;
    use mysql::prelude::*;
    use std::collections::HashMap;
    use super::super::server::server::Blog;

    /// 获取 blog 目录
    pub fn get_index() -> HashMap<String, String> {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 查询
        let res: Vec<(String, String)> = conn.query("select name, descript from blog order by id").unwrap();
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

    pub fn add_blog(blog: Blog) -> bool {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 添加
        let mut query = String::new();
        query.push_str("INSERT INTO blog (name, descript,  article)\n");
        query.push_str("VALUES ('");
        query.push_str(&blog.blog_name);
        query.push_str("', '");
        query.push_str(&blog.desc);
        query.push_str("', '");
        query.push_str(&blog.value);
        query.push_str("')");
        if let Err(_) = conn.query_drop(query) {
            return false;
        }
        // 返回结果
        true
    }

    pub fn delete_blog(name: String) -> bool {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 添加
        let mut query = String::new();
        query.push_str("delete from blog\n");
        query.push_str("where name=\"");
        query.push_str(&name);
        query.push_str("\"");
        if let Err(_) = conn.query_drop(query) {
            return false;
        }
        // 返回结果
        true
    }
}

#[cfg(test)]
mod database_tests {
    use super::database::*;
    use super::super::server::server::Blog;

    #[test]
    fn test_index() {
        let res = get_index();
        println!("result from database/myblog/blog:");
        println!("{:?}", res);
    }

    #[test]
    fn test_addblog() {
        let blog = Blog {
            id: 1,
            blog_name: String::from("aaa"),
            desc: String::from("bbbbb"),
            value: String::from("ccccc")
        };
        let res = add_blog(blog);
        assert_eq!(res, true);
    }

    #[test]
    fn test_deleteblog() {
        let res = delete_blog(String::from("aaa"));
        assert_eq!(res, true);
    }
}