use std::{cell::Cell, sync::{Arc, atomic::{AtomicUsize, Ordering}}};

use actix_web::{web, Responder, HttpServer, App};


#[derive(Clone)]
struct AppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!("global_count: {}\nlocal_count: {}", 
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get())
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);
    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);
    format!("global_count: {}\nlocal_count: {}", 
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get())
}

//visit  GET POST 
//127.0.0.1:8080
//127.0.0.1:8080/add

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let data = AppState {
        local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0))
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .route("/", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
