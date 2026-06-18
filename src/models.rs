use serde::{Serialize, Deserialize};
//use chrono::Months;
//use chrono::Weekday;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status{
    Pending,
    Completed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Category{
    Sports,
    Learning,
    Fitness,
    Cognitive,
    Other(String),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Months{
    Month(u8),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Weekday{
    WeekDay(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Occurence{
    Daily,
    Weekly(Vec<Weekday>),
    Monthly(Vec<Months>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Habit{
    pub name: String,
    pub purpose: String,
    pub category: Category,
    pub status: Status,
    pub occurence: Occurence,
}   

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User{
    pub name: String,
    pub id: String,
    pub age: u16,
    pub password: String,
    pub habit: Vec<Habit>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AppData{
    pub user: Vec<User>,
}

impl User{
    pub fn create_user(name: String, id: String, age: u16, password: String, habits: Vec<Habit>)-> User{
        let user = User{
            name,
            id,
            age,
            password,
            habit: habits,
        };
        let appdata = AppData{
            user: vec![user.clone()],
        };       
        user
        
    }
    pub fn display(&self){
        println!("Name: {}", self.name);
        println!("ID: {}", self.id);
        println!("Age: {}", self.age);
        println!();
        for habit in &self.habit{
            println!("Habit Name: {0:}", habit.name);
            println!("Habit Purpose: {0:}", habit.purpose);
            println!("Habit Category: {:?}", habit.category);
            println!("Habit Status: {:?}", habit.status);
            println!("Habit Occurence: {:?}", habit.occurence);
            println!()
        }
    }
}
