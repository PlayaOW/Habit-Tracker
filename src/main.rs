mod models;
mod habit;
mod storage;
use models::*;
use crate::models::*;
use storage::*;

//Need a lot more work on main.
// Especially for taking user input and linking them to the right data.
// main.rs is primarily being used for testing purposes now.
fn main() {
    let habit1: Habit = models::Habit::create_habit(String::from("Learning Rust"), String::from("Eventually build CLI tools"), Category::Learning, Status::Pending, Occurence::Daily);
    let habit = models::Habit::create_habit(String::from("Learning Math"), String::from("To get to Calc 3"), Category::Learning, Status::Pending, Occurence::Daily);
    let occureces = Occurence::create_occurence("Weekly");
    let habit02: Habit = Habit::create_habit(String::from("Shower"), String::from("Stay Clean"), Category::Other(String::from("Cleaniness")), Status::Pending, occureces);
    let mut habitList: Vec<Habit> = Vec::new();
    habitList.push(habit);
    habitList.push(habit1);
    habitList.push(habit02);
    //println!("Occurence: {:?}", occureces);
    let ray = models::User::create_user(String::from("Ray"), String::from("Ray880"), 29, String::from("Hush"), habitList);
    ray.display();
    Habit::list_habits(&ray);
    AppData::update_json("./habit_data/data.json", &ray).expect("Failed to update JSON");
    AppData::find_usr("./habit_data/data.json", "Ray880");
}
