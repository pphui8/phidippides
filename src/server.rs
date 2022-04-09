//! src/server.rs
//! 
//! 使用rocket框架对网络请求进行异步处理

pub mod server {
    use rocket::{get, post, catch, catchers};
    use rocket::routes;
    use rocket::serde::json::serde_json::json;
    use rocket::serde::json::serde_json::to_string;
    use rocket::serde::{Serialize, Deserialize};
    use rocket::serde::json::{Json, Value};
    use super::super::database;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Blog {
        pub token: String,
        pub blog_name: String,
        pub desc: String,
        pub value: String,
        pub tag: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    #[serde(crate = "rocket::serde")]
    pub struct Comment {
        pub token: String,
        pub username: String,
        pub url: String,
        pub value: String,
        pub time: String,
    }

    /// root 路径测试
    #[get("/")]
    async fn index() -> Value {
        json!({
            "status": 200
        })
    }

    /// 获取 blog 目录
    #[get("/")]
    async fn get_index() -> Value {
        let res = database::database::get_index();
        json!({
            "index": to_string(&res).unwrap()
        })
    }

    /// 获取具体 blog
    #[get("/<blog_name>")]
    async fn get_blog(blog_name: String) -> Value {
        let res: String = database::database::get_blog(blog_name);
        json!({
            "status": 200,
            "blog_root": to_string(&res).unwrap()
        })
    }

    /// 根据 blog 名删除一个blog
    #[get("/<blog_name>/<token>")]
    async fn del_blog(blog_name: String, token: String) -> Value {
        if String::from("pphui8") != token {
            return json!({
                "status": "failed",
                "error": "wrong token",
            })
        }
        let res = database::database::delete_blog(blog_name);
        if res {
            json!({
                "status": "success"
            })
        } else {
            json!({
                "status": "failed",
                "error": "fail to delete blog",
            })
        }
    }

    /// 获取comment（全部）
    #[get("/")]
    async fn get_comment() -> String {
        let res = database::database::get_comment();
        to_string(&res).unwrap()
    }

    /// 获取归档
    #[get("/")]
    async fn get_filing() -> String {
        let res = database::database::count_tags();
        to_string(&res).unwrap()
    }

    /// 添加一个 blog
    #[post("/", format = "json", data = "<blog>")]
    async fn add_blog(blog: Json<Blog>) -> Value {
        let recv = blog.into_inner();
        if recv.token != String::from("pphui8")  {
            return json!({
                "status": "failed",
                "error": "wrong token"
            })
        }
        let res = database::database::add_blog(recv);
        if res {
            json!({
                "status": "success"
            })
        } else {
            json!({
                "status": "failed"
            })
        }
    }

    /// 添加一个 comment
    #[post("/", format = "json", data = "<comment>")]
    async fn add_comment(comment: Json<Comment>) -> Value {
        let recv = comment.into_inner();
        if recv.token != String::from("pphui8")  {
            return json!({
                "status": "failed",
                "error": "wrong token"
            })
        }
        let res = database::database::add_comment(recv);
        if res {
            json!({
                "status": "success"
            })
        } else {
            json!({
                "status": "failed"
            })
        }
    }

    /// 404 处理函数
    #[catch(404)]
    async fn not_fount() -> Value {
        json!({
            "status": 404,
            "error": "not found"
        })
    }

    /// 404 处理函数
    #[catch(500)]
    async fn internal_server_error() -> Value {
        json!({
            "status": 500,
            "error": "blog not found"
        })
    }

    /// 400 处理函数
    #[catch(400)]
    async fn bad_request() -> Value {
        json!({
            "status": 400,
            "error": "Bad Request"
        })
    }

    /// 422 处理函数
    #[catch(422)]
    async fn unprocessable_entity() -> Value {
        json!({
            "status": 422,
            "error": "Unprocessable Entity"
        })
    }

    #[rocket::main]
    pub async fn server_start() -> Result<(), Box<dyn std::error::Error>> {
        rocket::build()
            // get routers
            .mount("/", routes![index])
            .mount("/index", routes![get_index])
            .mount("/blog", routes![get_blog])
            .mount("/delblog", routes![del_blog])
            .mount("/filing", routes![get_filing])
            .mount("/comment", routes![get_comment])
            // post routers
            .mount("/addblog", routes![add_blog])
            .mount("/addcomment", routes![add_comment])
            // cathers
            .register("/", catchers![not_fount, unprocessable_entity, bad_request, internal_server_error])
            .launch()
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::server::*;
    #[test]
    fn launch() {
        println!("启动服务程序...");
        server_start().unwrap();
    }
}