use actix_web::{web, Result, get, post, HttpServer, App, HttpRequest};
use serde::Deserialize;

//visit
//127.0.0.1:8080/users/654321/free
//127.0.0.1:8080?user_name=Spark
//127.0.0.1:8080/submit
//  POST
//  {
//     "user_name": "free"
//  }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index)
                                 .service(submit)
                                 .service(users))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/users/{user_id}/{friend}")]
async fn users(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let user_id: i32 = req.match_info().query("user_id").parse().unwrap();
    Ok(format!("Welcome {}, user_id {}!", name, user_id))
}

#[derive(Deserialize)]
struct Info {
    user_name: String,
}

#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.user_name)
}

#[post("/submit")]
async fn submit(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.user_name))
}