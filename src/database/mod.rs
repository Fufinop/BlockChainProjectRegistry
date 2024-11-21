use crate::models::project::Project;
use mongodb::{options::ClientOptions, Client};
use std::sync::Arc;

pub async fn init() -> Arc<mongodb::Collection<Project>> {
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI no configurado en .env");
    let mongo_db = std::env::var("MONGO_DB").expect("MONGO_DB no configurado en .env");

    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Error al parsear la URI de MongoDB");
    let client = Client::with_options(client_options).expect("Error al crear cliente de MongoDB");

    let db = client.database(&mongo_db);
    Arc::new(db.collection::<Project>("projects"))
}
