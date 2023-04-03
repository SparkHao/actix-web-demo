use actix_web::{HttpServer, App, web, Responder};

async fn index() -> impl Responder {
    "Hello world!"
}

//visit
//http://127.0.0.1:8080/app/index.html

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
