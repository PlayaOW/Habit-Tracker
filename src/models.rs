//use chrono::Months;
//use chrono::Weekday;
//use serde::Serialize;
//use serde::Deserialize;

#[derive(Debug)]
pub enum Status{
    Pending,
    Completed,
}

#[derive(Debug)]
pub enum Category{
    Sports,
    Learning,
    Fitness,
    Cognitive,
    Other(String),
}
#[derive(Debug)]
pub enum Months{
    Month(u8),
}

#[derive(Debug)]
pub enum Weekday{
    WeekDay(String),
}

#[derive(Debug)]
pub enum Occurence{
    Daily,
    Weekly(Vec<Weekday>),
    Monthly(Vec<Months>),
}

#[derive(Debug)]
pub struct Habit{
    name: String,
    purpose: String,
    category: Category,
    status: Status,
    occurence: Occurence,
}   

#[derive(Debug)]
pub struct User{
    name: String,
    id: String,
    age: u16,
    password: String,
    habit: Vec<Habit>,
    //failed to realize at first that One person may have multiple habits
    // Reason why decided to implement Vector data type for Habit data type
}

#[derive(Debug, Default)]
struct AppData<'a>{
    user: Vec<&'a User>,
}

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
impl User{
    pub fn create_user(name: String, id: String, age: u16, password: String, habits: Vec<Habit>)-> User{
        let user = User{
            name,
            id,
            age,
            password,
            habit: habits, //This takes a vector data type as args and 
            // stores that for user as their habit
            // Associates a habit to a user.
        };
        user //returns user

    }
    pub fn display(&self){
        println!("Name: {}", self.name);
        println!("ID: {}", self.id);
        println!("Age: {}", self.age);
        for habit in &self.habit{
            println!("Habit Name: {0:}", habit.name);
            println!("Habit Purpose: {0:}", habit.purpose);
            println!("Habit Category: {:?}", habit.category);
            println!("Habit Status: {:?}", habit.status);
            println!("Habit Occurence: {:?}", habit.occurence);
        } //Use for loop to display all associated habit for a user.
    }
}
