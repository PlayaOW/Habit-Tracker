use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub enum Status{
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Category{
    Sports,
    Learning,
    Fitness,
    Cognitive,
    Other(String),
}
#[derive(Debug)]
pub struct Months{
    month: String,
}
#[derive(Debug)]
pub struct Weekday{
    weekday: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Occurence{
    Daily,
    Weekly(Vec<Weekday>),
    Monthly(Vec<Months>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Habit{
    name: String,
    purpose: String,
    category: Category,
    status: Status,
    occurence: Occurence,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    name: String,
    id: String,
    age: u16,
    password: String,
    habit: Habit,
}

//#[derive(Debug, Serialize, Deserialize)]
//struct AppData{
    //user: Vec(<User>),
//}


impl User{
    fn display(&self){
        println!("Name: {}", self.name);
        println!("ID: {}", self.id);
        println!("Age: {}", self.age);
        println!("Habit: {:?}", self.habit);
    }
}