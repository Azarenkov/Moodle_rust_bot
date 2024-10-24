// use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Grades {
    pub usergrades: Vec<GradeItems>
}

#[derive(Debug, Deserialize)]
pub struct GradeItems {
    pub gradeitems: Vec<Grade>
}

#[derive(Debug, Deserialize)]
pub struct Grade {
    pub itemname: String,
    pub percentageformatted: String
}

pub(crate) trait GetGrades {
    async fn get_grades(token: &str, user_id: &str, course_id: &str) -> Result<Grades, reqwest::Error>;
}