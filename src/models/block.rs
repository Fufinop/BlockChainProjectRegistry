use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub project_id: String,
    pub description: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub participants: Vec<String>,
    pub status: String,
    pub previous_hash: String,
    pub hash: String,
}
