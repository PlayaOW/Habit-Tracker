mod models;

use models::*;

use crate::models::Category::Learning;
fn main() {
    let habit = Habit{
        name: String::from("Learning Math"),
        purpose: String::from("To get to Calc 3"),
        category: Category::Learning,
        status: Status::Pending,
        occurence: Occurence::Daily,
    };

    let ray = User{
        name: String::from("Ray"),
        id: String::from("1101"),
        age: 29,
        password: String::from("Hash"),
        habit: habit,
    };

    ray.display();
}
