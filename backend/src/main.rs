use std::fs;

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

    async fn get_collection(&self, index: u32) -> Option<common::Collection> {
        if index == 1 {
            Some(common::Collection {
                name: "test collection".to_owned(),
                files: vec![common::File {
                    name: "test file".to_owned(),
                    index: 1,
                    kind: common::FileKind::Image,
                }],
            })
        } else {
            None
        }
    }

    async fn get_file_path(&self, index: u32) -> Option<String> {
        if index == 1 {
            Some(format!("./testdata/a.jpg"))
        } else {
            None
        }
    }
}

#[get("/")]
async fn test(_cx: web::Data<Context>) -> impl Responder {
    println!("test called");
    HttpResponse::Ok().json(common::TestResponse {
        message: "hello world from test".to_owned(),
    })
}

#[get("/collections")]
async fn collections(cx: web::Data<Context>) -> impl Responder {
    let cx = cx.get_ref();
    let collections = cx.get_collections().await;
    HttpResponse::Ok().json(collections)
}

#[get("/collection/{index}")]
async fn collection(index: web::Path<u32>, cx: web::Data<Context>) -> impl Responder {
    let cx = cx.get_ref();
    let index = index.into_inner();
    if let Some(collection) = cx.get_collection(index).await {
        HttpResponse::Ok().json(collection)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/file/{index}")]
async fn file(index: web::Path<u32>, cx: web::Data<Context>) -> impl Responder {
    // FIXME: this should return the file not URL
    let cx = cx.get_ref();
    let index = index.into_inner();
    if let Some(path) = cx.get_file_path(index).await {
        get_file_response(&path)
    } else {
        HttpResponse::NotFound().finish()
    }
}

fn get_file_response(path: &str) -> HttpResponse {
    match fs::read(path) {
        Ok(data) => HttpResponse::Ok().content_type("image/jpeg").body(data),
        Err(_) => HttpResponse::InternalServerError().body("failed to get image data"),
    }
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
            .service(collection)
            .service(file)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
