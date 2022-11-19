use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UserNumPwd {
    pub number: String,
    pub password: String,
}
