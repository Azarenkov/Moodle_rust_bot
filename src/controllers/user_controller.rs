use reqwest::Client;
use crate::models::user::GetUserData;
use crate::User;
use super::helpers::Functions;

impl GetUserData for User {
    async fn get_user_info(token: &str) -> Result<User, reqwest::Error> {
        let client = Client::new();
    
        let function = Functions::GetUserData.new();
    
        let url = format!(
            "https://moodle.astanait.edu.kz/webservice/rest/server.php?wstoken={}&wsfunction={}&moodlewsrestformat=json",
            token,
            function,
        );
    
        let response = client.get(&url).send().await?.json::<User>().await?;
        Ok(response)
    }
}
