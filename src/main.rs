mod models;

use models::*;

use crate::models::Category::Learning;
fn main() {
    let habit = models::Habit::create_habit(String::from("Learning Math"), String::from("To get to Calc 3"), Category::Learning, Status::Pending, Occurence::Daily);

    let ray = models::User::create_user(String::from("Ray"), String::from("1102"), 29, String::from("Hush"), habit);

    ray.display();
}
