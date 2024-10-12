use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FileKind {
    Image,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub index: usize,
    pub kind: FileKind,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionIdentifier {
    pub name: String,
    pub index: usize,
}
