use rocket::{serde::json::Json, State};
use std::sync::{Arc, Mutex};

use crate::models::blockchain::Blockchain;

#[get("/blockchain")]
pub fn get_blockchain(
    blockchain: &State<Arc<Mutex<Blockchain>>>,
) -> Json<Vec<crate::models::block::Block>> {
    let blockchain = blockchain.lock().unwrap();
    Json(blockchain.chain.clone())
}
