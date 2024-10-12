use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub message: String,
}
