use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileKind {
    Image,
    Video,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub name: String,
    pub index: usize,
    pub kind: FileKind,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
    pub name: String,
    pub files: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionIdentifier {
    pub name: String,
    pub index: usize,
}
