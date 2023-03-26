use crate::models::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GoogleResponse {
    pub code: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Jwt {
    pub id: Option<i32>,
    pub email: Option<String>,
}

impl Jwt {
    pub fn from_user(user: User) -> Self {
        Self {
            id: Some(user.id),
            email: Some(user.email),
        }
    }
}

#[derive(Serialize)]
pub struct OauthError(pub String);

#[derive(Serialize, Deserialize)]
pub struct IdToken {
    access_token: String,
    pub(crate) id_token: String,
}
