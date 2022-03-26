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
    struct Blog {
        id: usize,
        blog_name: String,
        desc: String,
        value: String,
        // comments: Vec<comment>
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
            "status": 200,
            "index": to_string(&res).unwrap()
        })
    }

    #[get("/<blog_name>")]
    async fn get_blog(blog_name: String) -> Value {
        let res: String = database::database::get_blog(blog_name);
        json!({
            "status": 200,
            "blog_root": to_string(&res).unwrap()
        })
    }

    #[post("/", format = "json", data = "<blog>")]
    async fn add_blog(blog: Json<Blog>) -> Value {
        let recv = blog.into_inner();
        println!("{:?}", recv);
        json!({
            "status": "success"
        })
    }

    /// 404 处理函数
    #[catch(404)]
    async fn not_fount() -> Value {
        json!({
            "status": 404,
            "error": "not found"
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
            .mount("/blog", routes![get_blog])
            .mount("/index", routes![get_index])
            // post routers
            .mount("/addblog", routes![add_blog])
            // cathers
            .register("/", catchers![not_fount, unprocessable_entity, bad_request])
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