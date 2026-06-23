//use chrono::Weekday;
use std::io;
use std::io::BufRead;
use crate::models::Occurence::Daily;
use crate::models::Occurence::Monthly;
use crate::models::Occurence::Weekly;
use crate::models::Weekday;
use crate::models::Habit;
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
            //println!("Habit Occurence: {:?}", habit.occurence);
            match &habit.occurence{
                Daily => println!("Habit Occurence: Daily"),
                Weekly(week) => {
                    for days in week{
                        println!("Weekdays: {:?}", days);
                    }
                }
                Monthly(months) => {
                    for month in months{
                        println!("Months: {:?}", month);
                    }
                }
            }
            println!()
        }
    }
}
impl Occurence{
    pub fn create_occurence(occurence_of_habit: &str)-> Occurence {
        // Would take an Occurence from user as in
        // Daily?
        // Weekly?
        // Monthly?
        // If "Daily", use match to return Occurence::Daily
        // If "Weekly" use 
        match occurence_of_habit.trim() {
            "Daily" => Occurence::Daily,
            "Weekly" => {
                println!("Enter days (e.g. Monday, Tuesday, Wednesday):");  //Just a basic println! to allow users to understand what is expected of them

                let input = io::stdin().lock().lines().next().unwrap().unwrap(); //Taking in user input using std::io
                // lock() aquires a mutex lock so that only this thread can read instead of multiple threads paritcipating
                // lines() comes from BufRead trait. Returns an iterator where each item is one line from stdin. It does not real all lines at once
                // it is lazy, reads one at a time as you pull from the iterator.
                // next() pulls the first item from the iterator and return Option<Item> since first item user enters may be empty
                // The 1st unwrap() unwraps the Option<> that was returned by next(). return Some(..) ifthere is something panics if None.
                // But then the item unwrapped is actually Result<String, io::Error> because reading from stdin can fail. So now you cannot do anything 
                // with Result type. Here comes the 2nd unwrap() that unwraps the Result type to get the actual String..

                let days: Vec<Weekday> = input.split(',').map(|s| Weekday::WeekDay(s.trim().to_string())).collect();
                // An iterator is anything that produces the next value in the sequence one at a time when asked.
                // Vectors are iterables.
                // Vectors implement IntoIterator, and using IntoIterator you can get the next value by vectorVariable.iter()
                // The mental model is
                // iterator.map(|object| /* transform object into something else */)
                // The map() function does not drive the iteration. It just describes what to do to each item as collect() drives it
                // What collect is essentially doing under the hood
                // let mut result = Vector::new();

                // loop {
                //     match iter.next(){
                //         Some(item) => result.push(item),
                //         None => break,
                //     }
                // }
                
                Occurence::Weekly(days)
            }
            _ => panic!("Unknown occurence: {}", occurence_of_habit),
        }

    }
}