

// use actix_web::{get, web, App, Error, HttpResponse, HttpServer,Either};
// use futures::{future::ok, stream::once};

// type DiffResponse = Either<HttpResponse,Result<&'static str, Error>>;


// const is_variant:bool = true;

// async fn index() -> DiffResponse {
//     if(is_variant){
//         Either::Left(HttpResponse::Ok().body("responded"))
//     }else{
//         Either::Right(Ok("ok response"))
//     }
// }



// #[get("/stream")]
// async fn stream() -> HttpResponse {
//     let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

//     HttpResponse::Ok()
//         .content_type("application/json")
//         .streaming(body)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(stream))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }