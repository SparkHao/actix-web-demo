use actix_web::{web, HttpResponse, HttpServer, App};

//visit
//127.0.0.1:8080
//127.0.0.1:8080/app
//127.0.0.1:8080/api/test

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("/") })
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),   
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
                    .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

