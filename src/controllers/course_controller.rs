use reqwest::Client;
use crate::models::course::GetCoursesData;
use crate::Course;
use super::helpers::Functions;

impl GetCoursesData for Course {
    async fn get_courses(token: &str) -> Result<Vec<Course>, reqwest::Error> {
        let client = Client::new();
        let function = Functions::GetAllCourses.new();

        let url = format!(
            // "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}",
            "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json&userid=19401", 
            token, 
            function,
        );
    
        let response = client.get(&url).send().await?.json::<Vec<Course>>().await?;
    
        Ok(response)
    }
}