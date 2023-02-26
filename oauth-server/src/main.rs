use actix_web::get;
use actix_web::http::header::{HeaderValue, LOCATION};
use actix_web::http::StatusCode;
use actix_web::web::{Json, Query};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use openidconnect::core::CoreClient;
use reqwest::Client;
use serde::{Deserialize, Serialize};

mod dbcontroller;
mod models;
mod schema;

static CLIENT_ID: &str = "526205543724-jkq58jp5ch15a754pbkilr4n2sh1lbka.apps.googleusercontent.com";
static CLIENT_SECRET: &str = "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";

#[derive(Serialize, Deserialize)]
pub struct GoogleResponse {
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct JWT {
    email: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IdToken {
    access_token: String,
    id_token: String,
}

#[get("/auth")]
async fn authorize() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .append_header((
            LOCATION,
            HeaderValue::from_static(
                "https://accounts.google.com/o/oauth2/v2/auth?\
                access_type=offline&\
                include_granted_scopes=true&\
                scope=openid%20email&\
                response_type=code&\
                redirect_uri=http://localhost:8080/test&\
                client_id=526205543724-jkq58jp5ch15a754pbkilr4n2sh1lbka.apps.googleusercontent.com",
            ),
        ))
        .finish()
}

#[get("/test")]
async fn test(query: Option<Query<GoogleResponse>>) -> impl Responder {
    let response_body = match query {
        Some(response) => format!(
            "code={}&\
            client_id={}&\
            client_secret={}&\
            redirect_uri=http://localhost:8080/test&\
            grant_type=authorization_code",
            response.code.trim(),
            CLIENT_ID,
            CLIENT_SECRET
        ),
        None => {
            return Json(JWT {
                email: None,
                error: Some("Authorization error".to_string()),
            })
        }
    };

    let client = Client::new();
    let id_token_request = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(response_body)
        .send();
    // println!("{}", id_token_request.await.unwrap().text().await.unwrap());

    let mut response_content = String::new();
    if let Ok(response) = id_token_request.await {
        if let Ok(res) = response.text().await {
            response_content = res;
        }
    }
    // println!("{}", response_content);
    let jwt = match serde_json::from_str::<IdToken>(response_content.as_str()) {
        Ok(json) => get_id_token_jwt(json).await,
        Err(_) => Err("Parsing id_token error".to_string()),
    };
    match jwt {
        Ok(val) => Json(val),
        Err(str) => Json(JWT {
            email: None,
            error: Some(str),
        }),
    }
}

pub async fn get_id_token_jwt(id_token_response: IdToken) -> Result<JWT, String> {
    let client = Client::new();
    let id_token_info_request = client
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&[("id_token", id_token_response.id_token)])
        .send()
        .await;
    // println!("{}", id_token_info_request.unwrap().text().await.unwrap());
    if let Ok(response) = id_token_info_request {
        if let Ok(text) = response.text().await {
            return Ok(serde_json::from_str::<JWT>(text.as_str()).unwrap());
        }
    }
    Err("Something wrong".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(authorize).service(test))
        .bind(("127.0.0.1", 8080))
        .expect("Cannot run application on port 8080")
        .run()
        .await
}
