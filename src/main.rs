use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde_json::json;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix-web!")
}

async fn json_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": "John Doe",
        "age": 43,
        "is_student": false
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/json", web::get().to(json_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
