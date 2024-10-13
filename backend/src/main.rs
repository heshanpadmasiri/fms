use std::{fs, path::Path, usize};

use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

// TODO: lazy intialize these file information
// TODO: use a config file to tell where we should star the search
static BASE_DIR: &str = "./testdata";
#[derive(Clone)]
struct Context {
    collections: Vec<common::Collection>,
    file_paths: Vec<String>,
}

fn create_context_at(path: &Path) -> Result<Context, String> {
    let mut collections_vec = Vec::new();
    let mut file_paths = Vec::new();
    let mut top_level_collection = common::Collection {
        name: String::from("Root"),
        files: Vec::new(),
    };
    for entry in fs::read_dir(path).map_err(|err| format!("failed to read dir due to {:?}", err))? {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let (c, p) = create_collection(&path, file_paths.len())?;
            collections_vec.push(c);
            file_paths.extend(p);
        } else {
            // TODO: factor this and corresponding code in create_collection out
            let index = file_paths.len();
            let f = create_file(&path, index)?;
            if !is_supported(&f) {
                continue;
            }
            top_level_collection.files.push(f);
            file_paths.push(
                path.to_str()
                    .ok_or(String::from("failed to convert path to string"))?
                    .to_owned(),
            );
            dbg!(&file_paths[index]);
        }
    }
    if !top_level_collection.files.is_empty() {
        collections_vec.push(top_level_collection);
    }
    Ok(Context {
        collections: collections_vec,
        file_paths,
    })
}

fn create_collection(
    path: &Path,
    next_index: usize,
) -> Result<(common::Collection, Vec<String>), String> {
    let mut files = Vec::new();
    let mut file_paths = Vec::new();
    for entry in fs::read_dir(path).map_err(|err| format!("failed to read dir due to {:?}", err))? {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // TODO: Think about how to deal with nested collections
            continue;
        }
        let f = create_file(&path, next_index + files.len())?;
        if !is_supported(&f) {
            continue;
        }
        files.push(f);
        file_paths.push(
            path.to_str()
                .ok_or(String::from("failed to convert path to string"))?
                .to_owned(),
        );
    }
    Ok((
        common::Collection {
            name: path.file_name().unwrap().to_str().unwrap().to_owned(),
            files,
        },
        file_paths,
    ))
}

fn is_supported(f: &common::File) -> bool {
    match f.kind {
        common::FileKind::Image => true,
        common::FileKind::Video => true,
        _ => false,
    }
}

fn create_file(path: &Path, index: usize) -> Result<common::File, String> {
    let name = path
        .file_name()
        .ok_or(String::from("failed to get file name"))?
        .to_str()
        .ok_or(String::from("failed to convert file name to string"))?
        .to_owned();
    let kind: common::FileKind = match path.extension() {
        Some(ext) => {
            if ext == "jpg" || ext == "jpeg" {
                common::FileKind::Image
            } else if ext == "mp4" {
                common::FileKind::Video
            } else {
                common::FileKind::Other
            }
        }
        None => common::FileKind::Other,
    };
    Ok(common::File { name, index, kind })
}

impl Context {
    async fn get_collections(&self) -> Vec<common::CollectionIdentifier> {
        self.collections
            .iter()
            .enumerate()
            .map(|(i, c)| common::CollectionIdentifier {
                name: c.name.clone(),
                index: i,
            })
            .collect()
    }

    async fn get_collection(&self, index: usize) -> Option<common::Collection> {
        if index < self.collections.len() {
            Some(self.collections[index].clone())
        } else {
            None
        }
    }

    async fn get_file_path(&self, index: usize) -> Option<String> {
        if index < self.file_paths.len() {
            dbg!(&self.file_paths);
            Some(self.file_paths[index].clone())
        } else {
            None
        }
    }
}

#[get("/collections")]
async fn collections(cx: web::Data<Context>) -> impl Responder {
    let cx = cx.get_ref();
    let collections = cx.get_collections().await;
    HttpResponse::Ok().json(collections)
}

#[get("/collection/{index}")]
async fn collection(index: web::Path<usize>, cx: web::Data<Context>) -> impl Responder {
    let cx = cx.get_ref();
    let index = index.into_inner();
    if let Some(collection) = cx.get_collection(index).await {
        dbg!(&collection);
        HttpResponse::Ok().json(collection)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[get("/file/{index}")]
async fn file(index: web::Path<usize>, cx: web::Data<Context>) -> impl Responder {
    // FIXME: this should return the file not URL
    let cx = cx.get_ref();
    let index = index.into_inner();
    dbg!(index);
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
            .app_data(web::Data::new(
                create_context_at(Path::new(BASE_DIR)).unwrap(),
            ))
            .service(collections)
            .service(collection)
            .service(file)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
