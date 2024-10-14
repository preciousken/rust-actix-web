
use actix_web::{web, App, HttpServer, Responder,HttpResponse};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Api is fully responsive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/api/v1")
                // ...so this handles requests for `GET /api/v1/status`
                .route("/status", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}