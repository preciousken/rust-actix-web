use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

// constants
struct Response {
    status: bool,
    message: &'static str,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
async fn index() -> impl Responder {
    "Hello world!"
}

async fn manual_hello() -> impl Responder {
    let response = json!({
        "status": true,
        "message": "done..."
    });

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}