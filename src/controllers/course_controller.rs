use reqwest::Client;
use crate::models::course::GetCoursesData;
use crate::Course;
use super::helpers::Functions;

impl GetCoursesData for Course {
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
}