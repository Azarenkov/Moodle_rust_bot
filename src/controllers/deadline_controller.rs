use reqwest::Client;
use crate::models::deadline::{GetDeadline, Events};
use super::helpers::Functions;

impl GetDeadline for Events {
    
    async fn get_deadline(token: &str) -> Result<Events, reqwest::Error> {
        let client = Client::new();
    
        let url = format!(
            "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json",
            token,
            Functions::GetDeadlines.new(),
        );
    
        let response = client.get(&url).send().await?.json::<Events>().await?;
    
        Ok(response)
    }

}