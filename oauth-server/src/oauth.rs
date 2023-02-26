use crate::oauth_models::{IdToken, Jwt};
use reqwest::Client;

pub async fn get_id_token(response_body: String) -> Result<String, &'static str> {
    let client = Client::new();
    let id_token_request = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(response_body)
        .send();
    if let Ok(response) = id_token_request.await {
        if let Ok(res) = response.text().await {
            return Ok(res);
        }
    }
    Err("ID_TOKEN get error")
}

pub async fn get_id_token_jwt(id_token_response: IdToken) -> Result<Jwt, &'static str> {
    let client = Client::new();
    let id_token_info_request = client
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", id_token_response.id_token)])
        .send()
        .await;
    // println!("{}", id_token_info_request.unwrap().text().await.unwrap());
    if let Ok(response) = id_token_info_request {
        if let Ok(text) = response.text().await {
            return Ok(serde_json::from_str::<Jwt>(text.as_str()).unwrap());
        }
    }
    Err("Something wrong")
}
