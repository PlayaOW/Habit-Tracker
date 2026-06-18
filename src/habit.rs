use super::models::Habit;
use super::models::Category;
use super::models::Status;
use super::models::Occurence;

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
}