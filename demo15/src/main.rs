use actix_web::{guard, web, App, HttpResponse, HttpServer};

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

//visit
//127.0.0.1:8080/user/user_detail
//127.0.0.1:8080/prefix

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/prefix").to(index))
            .service(
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(HttpResponse::Ok))
                    .route(web::put().to(HttpResponse::Ok)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}