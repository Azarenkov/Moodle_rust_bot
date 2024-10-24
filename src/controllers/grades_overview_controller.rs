use reqwest::Client;
use crate::models::grades_overview::{GetGradesOverview, GradesOverview};
use super::helpers::Functions;

impl GetGradesOverview for GradesOverview {
    
    async fn get_grades_overview(token: &str) -> Result<GradesOverview, reqwest::Error> {
        let client = Client::new();
    
        let url = format!(
            "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json",
            token,
            Functions::GetGradesOverview.new(),
        );
    
        let response = client.get(&url).send().await?.json::<GradesOverview>().await?;
    
        Ok(response)
    }

}