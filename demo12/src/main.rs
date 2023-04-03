use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder, HttpServer, web, App, get, Error, Either
};
use serde::Serialize;
use futures::{stream::once, future::ok};


#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

// Responder
impl Responder for MyObj {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/")]
async fn index() -> impl Responder {
    MyObj { name: "user" }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(move || {
        App::new()
            // .route("/", web::to(index))
            .service(index)
            .service(stream)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));
    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}


// type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

// async fn index2() -> RegisterResult {
//     if is_a_variant() {
//         // choose Left variant
//         Either::Left(HttpResponse::BadRequest().body("Bad data"))
//     } else {
//         // choose Right variant
//         Either::Right(Ok("Hello!"))
//     }
// }