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
    habit: Habit,
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
    pub fn create_user(name: String, id: String, age: u16, password: String, habit: Habit)-> User{
        let user = User{
            name,
            id,
            age,
            password,
            habit,
        };
        let mut data = AppData::default();
        data.user.push(&user);
        user

    }
    pub fn display(&self){
        println!("Name: {}", self.name);
        println!("ID: {}", self.id);
        println!("Age: {}", self.age);
        println!("Habit Name: {0:}", self.habit.name);
        println!("Habit Purpose: {0:}", self.habit.purpose);
        println!("Habit Category: {:?}", self.habit.category);
        println!("Habit Status: {:?}", self.habit.status);
        println!("Habit Occurence: {:?}", self.habit.occurence);
    }
}
