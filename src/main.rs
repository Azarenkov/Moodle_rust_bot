use serde::Deserialize;
use reqwest::Client;
use std::fmt::Display;
use serde_json::Value; // Используем serde_json для работы с JSON как с универсальной структурой
use tokio;

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    userid: i64,
    fullname: String,
}

#[derive(Debug, Deserialize)]
struct Course {
    id: i32,
    fullname: String,
    // category: String,
    completed: Option<bool>,
    // start_date: i64,
    // end_date: i64,
}

enum Functions {
    GetUserData,
    GetAllCourses,
}
impl Functions {
    fn new(&self) -> &'static str {
        match self {
            Functions::GetUserData => "core_webservice_get_site_info&moodlewsrestformat=json",
            Functions::GetAllCourses => "core_enrol_get_users_courses&moodlewsrestformat=json",
        }
    }
}

async fn get_courses(token: &str) -> Result<Vec<Course>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .build()?;

    let url = format!(
        // "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}",
        "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken=711abc349948337f8b97cbb01b76adf5&wsfunction=core_enrol_get_users_courses&moodlewsrestformat=json&userid=19401",
        // token,
        // Functions::GetAllCourses.new(),
    );

    let response = client.get(&url).send().await?.json::<Vec<Course>>().await?;

    Ok(response)
}

async fn get_user_info(token: &str) -> Result<User, reqwest::Error> {
    let client = Client::new();

    let function = Functions::GetUserData.new();

    let url = format!(
        "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}",
        token,
        function,
    );

    let response = client.get(&url).send().await?.json::<User>().await?;
    Ok(response)
}


#[tokio::main]
async fn main() {
    let token = "711abc349948337f8b97cbb01b76adf5"; 
    // let user_ids = vec![1, 2]; 
    // Functions::new(&Functions::GetUserData);

    match get_user_info(token).await {
        Ok(user_response) => {
            println!("{}", user_response.fullname);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    match get_courses(token).await {
        Ok(courses) => {
            for course in courses {
                println!("Course ID: {}, Full Name:{} , Completed:{}", course.id, course.fullname, course.completed.unwrap_or_default());
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
