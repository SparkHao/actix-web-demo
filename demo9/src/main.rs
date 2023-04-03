use actix_web::{HttpServer, web, error, HttpResponse, App, Responder};
use serde::{Deserialize, Serialize};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new().service(
            web::resource("/")
                .app_data(json_config)
                .route(web::post().to(index)),   
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Serialize, Deserialize)]
struct Info {
    user_name: String,
}

async fn index(info: web::Json<Info>) -> impl Responder {
    // format!("Welcome {}!", info.user_name)
    info
}