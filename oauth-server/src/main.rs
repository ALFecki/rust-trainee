use actix_web::http::header::{CONTENT_TYPE, HeaderValue, LOCATION};
use actix_web::http::{header, StatusCode};
use actix_web::web::{Query, Redirect};
use actix_web::{get, HttpRequest};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use actix_web::error::UrlencodedError::ContentType;
use serde::{Deserialize, Serialize};

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
                scope=https%3A//www.googleapis.com/auth/drive.metadata.readonly&\
                access_type=offline&\
                include_granted_scopes=true&\
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
    let response_body = format!("code={}&\
    client_secret=HMVctrTicW6RC1Q8T&\
    redirect_uri=http://localhost:8080/test2&\
    grant_type=authorization_code", query.unwrap().code);
    HttpResponse::Found().append_header((LOCATION, "https://oauth2.googleapis.com/token"))
        .append_header((CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED))
        .body(response_body)

}

#[get("/test2")]
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
