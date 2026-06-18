use crate::models::{AppData, User};
use std::fs;

impl AppData {
    pub fn update_json(path: &str, new_item: &User) -> Result<(), Box<dyn std::error::Error>> {
        // Read the existing JSON data from the file
        // If the file doesn't exist yet, we fall back to a default empty AppData
        let mut data: AppData = match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => AppData::default(),
        };

        // Deserialize into struct
        // what does this mean??
        // when in rust, you are using something like std::read to read a file
        // It reads the file as a string, considering you are reading the file
        // so that you can data from it and then use it in one of your data type such as struct
        // But struct does not use String for all of its fields. Seems like a problem right?
        // So we need to convert teh string into the struct field type and that is where Deseialize comes in. 
        // It takes the string and converts it into the struct field type.
        // By defining Rust struct and adding serde attributes, you can easily convert between JSON and Rust data structures, making it easier to work with JSON data in your Rust applications.
        
        // Fix: Push a clone of the User struct directly instead of a String
        data.user.push(new_item.clone());

        // Serialize the updated struct back to JSON
        let updated_json = serde_json::to_string_pretty(&data)?;

        // Write the new JSON string back to the file
        fs::write(path, updated_json)?;

        Ok(())
    }
}