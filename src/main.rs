use serde::Deserialize;
use reqwest::Client;
use std::fmt::Display;
use serde_json::Value; // Используем serde_json для работы с JSON как с универсальной структурой
use tokio;
// use firebase_rs::*;

mod models;
mod controllers;
use models::course::{GetCoursesData ,Course};
use models::user::{GetUserData, User};

#[tokio::main]
async fn main() {
    let token = "711abc349948337f8b97cbb01b76adf5"; 
    // let user_ids = vec![1, 2]; 
    // Functions::new(&Functions::GetUserData);
    match User::get_user_info(token).await {
        Ok(user_response) => {
            println!("{}", user_response.fullname);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    match Course::get_courses(token).await {
        Ok(courses) => {
            for course in courses {
                println!("Course ID: {}, Full Name:{} , Completed:{}", course.id, course.fullname, course.completed.unwrap_or_default());
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
