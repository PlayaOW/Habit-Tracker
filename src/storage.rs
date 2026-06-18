use crate::models::AppData;
use serde_json;
use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::error::Error;

impl AppData {
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let parent = std::path::Path::new(path).parent();
        if let Some(p) = parent {
            std::fs::create_dir_all(p)?;
        }
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<AppData, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: AppData = serde_json::from_reader(reader)?;
        Ok(data)
    }
}