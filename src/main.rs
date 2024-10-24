// use serde::Deserialize;
// use reqwest::Client;
// use std::fmt::Display;
// use serde_json::Value;
use tokio;
// use firebase_rs::*;

mod models;
mod controllers;

use models::{course, user, deadline, grade, grades_overview};
use course::{GetCoursesData ,Course};
use user::{GetUserData, User};
use deadline::{Events, GetDeadline};
use grade::{GetGrades, Grades};
use grades_overview::{GetGradesOverview, GradesOverview};


#[tokio::main]
async fn main() {
    let token = "711abc349948337f8b97cbb01b76adf5"; 
    let user_id = "19401";
    let course_id = "5257";

    match User::get_user_info(token).await {
        Ok(user_response) => {
            println!("{} {}", user_response.fullname, user_response.userid);
            println!("{}","-".repeat(150));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    match Course::get_courses(token).await {
        Ok(courses) => {
            for course in courses {
                println!("Course ID: {}, Full Name:{} , Completed:{}", course.id, course.fullname, course.completed.unwrap_or_default());
            }
            println!("{}","-".repeat(150));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match Events::get_deadline(token).await {
        Ok(deadlines) => {
            println!("{:#?}", deadlines);
            println!("{}","-".repeat(150));
        }
        Err(e) => eprintln!("Error: {}", e),
    }
   
    match Grades::get_grades(token, user_id, course_id).await {
        Ok(grades) => {
            println!("{:#?}", grades);
            println!("{}","-".repeat(150));
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    match GradesOverview::get_grades_overview(token).await {
        Ok(grades_overview) => {
            println!("{:#?}", grades_overview);
            println!("{}","-".repeat(150));
        }
        Err(e) => eprintln!("Error : {}", e),
    }
}
