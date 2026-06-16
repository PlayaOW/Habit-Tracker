mod models;

use models::*;

//Need a lot more work on main.
// Especially for taking user input and linking them to the right data.
// main.rs is primarily being used for testing purposes now.
fn main() {
    let mut habit = models::Habit::create_habit(String::from("Learning Math"), String::from("To get to Calc 3"), Category::Learning, Status::Pending, Occurence::Daily);
    let mut habitList: Vec<Habit> = Vec::new();
    habitList.push(habit);
    let ray = models::User::create_user(String::from("Ray"), String::from("1102"), 29, String::from("Hush"), habitList);

    ray.display();
}
