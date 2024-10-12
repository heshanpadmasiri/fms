use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[derive(Clone)]
struct Context {}

impl Context {
    async fn get_collections(&self) -> Vec<common::CollectionIdentifier> {
        vec![common::CollectionIdentifier {
            name: "test collection".to_owned(),
            index: 1,
        }]
    }
}

#[get("/")]
async fn test(_cx: web::Data<Context>) -> impl Responder {
    println!("test called");
    HttpResponse::Ok().json(common::TestResponse {
        message: "hello world from test".to_owned(),
    })
}

// TODO: figure out how to pass in a context here
#[get("/collections")]
async fn collections(cx: web::Data<Context>) -> impl Responder {
    let cx = cx.get_ref();
    let collections = cx.get_collections().await;
    HttpResponse::Ok().json(collections)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(web::Data::new(Context {}))
            .service(test)
            .service(collections)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
