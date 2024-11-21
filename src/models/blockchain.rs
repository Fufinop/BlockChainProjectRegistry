use crate::models::block::Block;
use crate::models::project::Project;
use chrono::Utc;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            project_id: "Genesis Block".to_string(),
            description: "Bloque inicial de la cadena".to_string(),
            start_date: "N/A".to_string(),
            end_date: None,
            participants: vec![],
            status: "N/A".to_string(),
            previous_hash: "0".to_string(),
            hash: Self::calculate_hash(0, Utc::now().timestamp(), "0", "Genesis Block"),
        };
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, project: &Project) {
        let previous_block = self.chain.last().unwrap();
        let index = previous_block.index + 1;
        let timestamp = Utc::now().timestamp();
        let hash = Self::calculate_hash(index, timestamp, &previous_block.hash, &project.name);
        let new_block = Block {
            index,
            timestamp,
            project_id: project.id.as_ref().unwrap().to_hex(),
            description: project.description.clone(),
            start_date: project.start_date.clone(),
            end_date: project.end_date.clone(),
            participants: project.participants.clone(),
            status: project.status.clone(),
            previous_hash: previous_block.hash.clone(),
            hash,
        };
        self.chain.push(new_block);
    }

    fn calculate_hash(index: u64, timestamp: i64, previous_hash: &str, data: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, previous_hash, data);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}
