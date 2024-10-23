use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct User {
    pub username: String,
    pub userid: i64,
    pub fullname: String,
}

pub(crate) trait GetUserData {
     async fn get_user_info(token: &str) -> Result<User, reqwest::Error>;
}