use std::string::ToString;
use actix_web::error::UrlencodedError::ContentType;
use actix_web::http::header::{HeaderValue, CONTENT_TYPE, LOCATION};
use actix_web::http::{header, StatusCode};
use actix_web::web::{Query, Redirect};
use actix_web::{get, HttpRequest};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};

static CLIENT_ID: &str = "526205543724-jkq58jp5ch15a754pbkilr4n2sh1lbka.apps.googleusercontent.com";
static CLIENT_SECRET: &str = "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";


#[derive(Serialize, Deserialize)]
pub struct GoogleResponse {
    code: String,
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
    // println!("{}", query.unwrap().code);
    let response_body = format!(
        "code={}&\
        client_id={}&\
        client_secret={}&\
        redirect_uri=http://localhost:8080/test&\
        grant_type=authorization_code",
        query.unwrap().code.trim(),
        CLIENT_ID,
        CLIENT_SECRET
    );
    let client = Client::new();
    let response = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(response_body)
        .send()
        .await
        .unwrap();


    let response_content = response.text().await.unwrap();
    println!("{}", response_content);
    HttpResponse::Ok().body(response_content)

    // HttpResponse::Ok().finish()
}

#[get("/test_response")]
async fn test2(query: Query<String>) -> impl Responder {
    println!("{}", query);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(authorize).service(test).service(test2))
        .bind(("127.0.0.1", 8080))
        .expect("Cannot run application on port 8080")
        .run()
        .await
}
