//! src/database.rs
//! 
//! 

pub mod database {
    use mysql::*;
    use mysql::prelude::*;
    use rocket::serde::Serialize;
    use super::super::server::server::Blog;

    #[derive(Serialize, Debug)]
    pub struct Index {
        id: usize,
        name: String,
        descript: String,
        tag: String,
    }

    #[derive(Serialize, Debug)]
    pub struct Comment {
        id: usize,
        username: String,
        url: String,
        value: String,
    }

    #[derive(Serialize, Debug)]
    pub struct Tags {
        test: usize,
        note: usize,
        blog: usize,
        code: usize,
    }

    /// 获取 blog 目录
    pub fn get_index() -> Vec<Index> {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 查询
        let res = conn.query_map(
            "select id, name, descript, tag from blog order by id",
            |(id, name, descript, tag)| Index {
                id,
                name,
                descript,
                tag,
            },
        ).expect("Query failed.");
        res
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
        query.push_str("INSERT INTO blog (name, descript, article, tag)\n");
        query.push_str("VALUES ('");
        query.push_str(&blog.blog_name);
        query.push_str("', '");
        query.push_str(&blog.desc);
        query.push_str("', '");
        query.push_str(&blog.value);
        query.push_str("', '");
        query.push_str(&blog.tag);
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

    pub fn count_tags() -> Tags {
        let mut tags = Tags {
            test: 0,
            note: 0,
            blog: 0,
            code: 0,
        };
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 查询
        let res: Vec<String> = conn.query("select tag from blog").unwrap();
        for tag in res {
            let tag = tag.as_str();
            match tag {
                "test" => tags.test += 1,
                "note" => tags.note += 1,
                "blog" => tags.blog += 1,
                "code" => tags.code += 1,
                _ => {}
            }
        }
        tags
    }

    pub fn get_comment() -> Vec<Comment> {
        // 获取链接
        let blog_url = "mysql://root:123212321@localhost:3306/myblog";
        let pool = Pool::new(blog_url).unwrap();
        let mut conn = pool.get_conn().unwrap();
        // 查询
        let res = conn.query_map(
            "select id, username, url, value from comment order by id",
            |(id, username, url, value)| Comment {
                id,
                username,
                url,
                value,
            },
        ).expect("Query failed.");
        res
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