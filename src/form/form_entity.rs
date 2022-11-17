use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLogin {
    pub number: String,
    pub password: String,
}
