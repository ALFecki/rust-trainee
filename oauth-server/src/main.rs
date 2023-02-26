use crate::dbcontroller::{create_user, database_connection, select_user};
use crate::models::NewUser;
use crate::oauth::{get_id_token, get_id_token_jwt};
use crate::oauth_models::{GoogleResponse, IdToken, Jwt};
use actix_web::get;
use actix_web::http::header::{HeaderValue, LOCATION};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Query};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use diesel::PgConnection;

use std::borrow::BorrowMut;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

mod oauth_models;

mod dbcontroller;
mod models;
mod oauth;
mod schema;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Found().append_header((LOCATION, "/auth")).finish()
}

#[get("/auth")]
async fn authorize() -> impl Responder {
    HttpResponse::Ok()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .append_header((
            LOCATION,
            HeaderValue::try_from(format!(
                "https://accounts.google.com/o/oauth2/v2/auth?\
                access_type=offline&\
                include_granted_scopes=true&\
                scope=openid%20email&\
                response_type=code&\
                redirect_uri={}&\
                client_id=526205543724-jkq58jp5ch15a754pbkilr4n2sh1lbka.apps.googleusercontent.com",
                &env::var("REDIRECT_URL").expect("redirect url must be setup")
            ))
            .unwrap(),
        ))
        .finish()
}

#[get("/login")]
async fn login(
    query: Option<Query<GoogleResponse>>,
    connection: Data<PostgresConnection>,
) -> impl Responder {
    let mut pg_connection = connection.lock().await;
    dotenv::dotenv().ok();
    let response_body = match query {
        Some(response) => format!(
            "code={}&\
            client_id={}&\
            client_secret={}&\
            redirect_uri={}&\
            grant_type=authorization_code",
            response.code.trim(),
            &env::var("CLIENT_ID").expect("client_id must be setup"),
            &env::var("CLIENT_SECRET").expect("client_secret must be setup"),
            &env::var("REDIRECT_URL").expect("redirect url must be setup")
        ),
        None => {
            return Json(Jwt::error("Authorization error"));
        }
    };

    let response_content = match get_id_token(response_body).await {
        Ok(text) => text,
        Err(str) => return Json(Jwt::error(str)),
    };
    let jwt = match serde_json::from_str::<IdToken>(response_content.as_str()) {
        Ok(json) => get_id_token_jwt(json).await,
        Err(_) => return Json(Jwt::error("Parsing id_token error")),
    };
    match jwt {
        Ok(val) => {
            let mut response = Jwt::default();
            if let Some(mail) = val.email {
                let user = NewUser::new(mail.clone());
                if !user.is_exists(pg_connection.borrow_mut()) {
                    response = Jwt::from_user(create_user(pg_connection.borrow_mut(), user))
                } else {
                    response =
                        Jwt::from_user(select_user(pg_connection.borrow_mut(), mail).unwrap())
                };
            };
            Json(response)
        }
        Err(str) => Json(Jwt::error(str)),
    }
}

type PostgresConnection = Arc<Mutex<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = Data::new(Arc::new(Mutex::new(
        database_connection().unwrap(),
    )));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&connection))
            .service(index)
            .service(authorize)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Cannot run application on port 8080")
    .run()
    .await
}
