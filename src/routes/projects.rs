use mongodb::{bson::oid::ObjectId, Collection};
use rocket::{serde::json::Json, State};
use std::sync::{Arc, Mutex};

use crate::models::{blockchain::Blockchain, project::Project};

pub type Db = Arc<Collection<Project>>;

#[post("/projects", format = "json", data = "<project>")]
pub async fn create_project(
    project: Json<Project>,
    db: &State<Db>,
    blockchain: &State<Arc<Mutex<Blockchain>>>,
) -> Result<&'static str, &'static str> {
    let mut project_with_id = project.clone();
    project_with_id.id = Some(ObjectId::new());

    if let Err(e) = db.insert_one(&*project_with_id).await {
        eprintln!("Error al guardar el proyecto: {:?}", e);
        return Err("Error al guardar el proyecto");
    }

    let mut blockchain = blockchain.lock().unwrap();
    blockchain.add_block(&project_with_id);

    Ok("Proyecto creado y añadido a la blockchain")
}
