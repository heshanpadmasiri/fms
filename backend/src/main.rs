use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TestResponse {
    message: String,
}

#[get("/")]
async fn test() -> impl Responder {
    println!("test called");
    HttpResponse::Ok().json(TestResponse {
        message: "hello world from test".to_owned(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(test)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
