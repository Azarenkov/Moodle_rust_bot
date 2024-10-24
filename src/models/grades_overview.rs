use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GradesOverview {
    grades: Vec<GradeOverview>
}

#[derive(Debug, Deserialize)]
pub struct GradeOverview {
    courseid: i32,
    grade: String,
    rawgrade: String,
}

pub(crate) trait GetGradesOverview {
    async fn get_grades_overview(token: &str) -> Result<GradesOverview, reqwest::Error>;
}