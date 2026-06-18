mod models;
mod storage;

use models::*;
use models::AppData;

//Need a lot more work on main.
// Especially for taking user input and linking them to the right data.
// main.rs is primarily being used for testing purposes now.
fn main() {
    let habit1: Habit = models::Habit::create_habit(String::from("Learning Rust"), String::from("Eventually build CLI tools"), Category::Learning, Status::Pending, Occurence::Daily);
    let habit = models::Habit::create_habit(String::from("Learning Math"), String::from("To get to Calc 3"), Category::Learning, Status::Pending, Occurence::Daily);
    let mut habitList: Vec<Habit> = Vec::new();
    habitList.push(habit);
    habitList.push(habit1);
    let ray = models::User::create_user(String::from("Ray"), String::from("1102"), 29, String::from("Hush"), habitList);
    ray.display();

    // persist app data to JSON
    let appdata = AppData { user: vec![ray] };
    match appdata.save_to_file("./habit_data/data.json") {
        Ok(_) => println!("Saved appdata to ./habit_data/data.json"),
        Err(e) => eprintln!("Failed to save appdata: {}", e),
    }

    // load it back and display
    match AppData::load_from_file("./habit_data/data.json") {
        Ok(loaded) => {
            println!("Loaded appdata from file:");
            for u in loaded.user.iter() {
                u.display();
            }
        }
        Err(e) => eprintln!("Failed to load appdata: {}", e),
    }
}
