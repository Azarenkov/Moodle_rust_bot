use reqwest::Client;
use crate::models::grade::{GetGrades, Grades};
use super::helpers::Functions;

impl GetGrades for Grades {
    async fn get_grades(token: &str, user_id: &str, course_id: &str) -> Result<Grades, reqwest::Error> {
        let client = Client::new();
        let function = Functions::GetGrades.new();

        let url = format!(
            "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json&userid={}&courseid={}", 
            token, 
            function,
            user_id,
            course_id
        );
    
        let response = client.get(&url).send().await?.json::<Grades>().await?;
    
        Ok(response)
    }
}