use actix_web::{web, Result, get, HttpServer, App};
use serde::Deserialize;

//visit
//127.0.0.1:8080/users/654321/free

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[get("/users/{user_id}/{friend}")]
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", info.friend, info.user_id))
}

