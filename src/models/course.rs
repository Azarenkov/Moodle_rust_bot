use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct Course {
    pub id: i32,
    pub fullname: String,
    // category: String,
    pub completed: Option<bool>,
    // start_date: i64,
    // end_date: i64,
}

pub(crate) trait GetCoursesData {
    async fn get_courses(token: &str) -> Result<Vec<Course>, reqwest::Error>;
}