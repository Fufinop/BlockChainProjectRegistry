#[macro_use]
extern crate rocket;
mod config;
mod database;
mod models;
mod routes;
mod utils;

use dotenvy::dotenv;
use std::sync::{Arc, Mutex};

#[rocket::launch]
async fn rocket() -> _ {
    dotenv().ok();

    let db = database::init().await;
    let blockchain = Arc::new(Mutex::new(models::blockchain::Blockchain::new()));

    rocket::build()
        .manage(db)
        .manage(blockchain)
        .mount("/", routes![routes::index])
        .mount(
            "/api",
            routes![
                routes::projects::create_project,
                routes::blockchain::get_blockchain
            ],
        )
}
