use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(test))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
