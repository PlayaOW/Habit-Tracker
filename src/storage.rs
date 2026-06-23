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
        for user in &data.user{
            if user.name == new_item.name{
                println!("User ALready Exist!!");
                println!("Your User ID is: {}", &new_item.id);
                return Ok(())
            }
        }
        data.user.push(new_item.clone());

        // Serialize the updated struct back to JSON
        let updated_json = serde_json::to_string_pretty(&data)?;

        // Write the new JSON string back to the file
        fs::write(path, updated_json)?;

        Ok(())
    }
    pub fn find_usr(path: &str, userID: &str)-> Option<User>{
        // This function invokes when user enters their user ID
        // This goes into the JSON data base to look for the user using their UID
        // If found it returns an User Object
        // Other returns None
        // **Logic** //
        // First open the file using File::open(path)
        // Turn the contents of file into string
        // Then Deserialize the String into struct AppData user vector
        // Iterate through user vector and use match to user.id to see
        // whether an user exist, if yes we return Oprion<User> 
        // This is then handled in main.rs logic to bring out the 
        // User object by extracting variable that is linked to match

        //let mut data = File::open(path)?;
        //let usr_data: String = String::from(data.read_to_string(&mut usr_data));

        //data.read_to_string(&mut usr_data);

         let ac_usr_data: AppData = match fs::read_to_string(path){
            Ok(usr_data) => serde_json::from_str(&usr_data).unwrap_or_default(),
            Err(_) => AppData::default(),
         };
         // This block of code here reads the data from the file and converts it to String
         // It assigns the converted String to Ok(var) to var..
         // Else It return AppData::default()

         for user in ac_usr_data.user{      // Iterates through AppData.user vector
            if &user.id == userID{     // Checks whether user.id matches with provided userID in fn param. Had to use to_string() since user.id is String and we are comparing str to String  
                println!("User Found: {}", user.name);
                return Some(user)
            }
         }
         println!("User Not Found!! Try Again");
         None

    }
}