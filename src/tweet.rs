use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Tweet {
    pub message: String,
    pub id: String,
    pub tag: String
}
