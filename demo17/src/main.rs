use actix_web::{get, web, App, HttpServer, Result};
use serde::Deserialize;

#[get("/{username}/{id}/index.html")] // <- define path parameters
async fn index(info: web::Path<(String, u32)>) -> Result<String> {
    let info = info.into_inner();
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

//visit
//127.0.0.1:8080/free/8888/index.html

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
            App::new().service(index)
                      .service(index2)
        )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

// extract path info using serde
#[get("/{username}/index.html")] // <- define path parameters
async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}