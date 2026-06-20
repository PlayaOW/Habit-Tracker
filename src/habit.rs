use super::models::Habit;
use super::models::Category;
use super::models::Status;
use super::models::Occurence;
use crate::models::User;

impl Habit{
    pub fn create_habit(name: String, purpose: String, category: Category, status: Status, occurence: Occurence)-> Habit{
        Habit{
            name,
            purpose,
            category,
            status,
            occurence,
        }
    }

    pub fn list_habits(user: &User){
        for habit in &user.habit{
            println!("Habit Name: {0:}", habit.name);
            println!("Habit Purpose: {0:}", habit.purpose);
            println!("Habit Category: {:?}", habit.category);
            println!("Habit Status: {:?}", habit.status);
            println!("Habit Occurence: {:?}", habit.occurence);
            println!()
        }
    }
}