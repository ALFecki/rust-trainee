use crate::dbcontroller::*;
use crate::models::NewUser;
use crate::oauth::*;
use crate::oauth_models::{GoogleResponse, IdToken, Jwt};
use actix_web::get;
use actix_web::http::header::{HeaderValue, LOCATION};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Query};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use diesel::PgConnection;

use std::borrow::BorrowMut;

use std::sync::Arc;
use tokio::sync::Mutex;
use yaml_config::load;

mod oauth_models;

mod dbcontroller;
mod models;
mod oauth;
mod schema;

#[derive(Clone, Debug)]
struct YamlData {
    redirect_url: String,
    client_id: String,
    client_secret: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Found()
        .append_header((LOCATION, "/auth"))
        .finish()
}

#[get("/auth")]
async fn authorize(config_data: Data<YamlData>) -> impl Responder {
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
                config_data.redirect_url
            ))
            .unwrap(),
        ))
        .finish()
}

#[get("/login")]
async fn login(
    query: Option<Query<GoogleResponse>>,
    connection: Data<PostgresConnection>,
    config_data: Data<YamlData>,
) -> impl Responder {
    let mut pg_connection = connection.lock().await;
    let response_body = match query {
        Some(response) => format!(
            "code={}&\
            client_id={}&\
            client_secret={}&\
            redirect_uri={}&\
            grant_type=authorization_code",
            response.code.trim(),
            config_data.client_id,
            config_data.client_secret,
            config_data.redirect_url
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
            if let Some(mail) = val.email {
                let user = NewUser::new(mail.as_str());
                let conn = pg_connection.borrow_mut();

                return Json(Jwt::from_user(match select_user(conn, mail.as_str()) {
                    Some(u) => u,
                    None => match create_user(conn, user) {
                        Ok(u) => u,
                        Err(str) => return Json(Jwt::error(str)),
                    },
                }));
            };
            Json(Jwt::error("Error with database operation"))
        }
        Err(str) => Json(Jwt::error(str)),
    }
}

type PostgresConnection = Arc<Mutex<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load("./config.yaml", None).expect("Cannot open config.yaml");

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        *config["DATABASE_USERNAME"]
            .as_string()
            .expect("USERNAME isn't set"),
        *config["DATABASE_PASSWORD"]
            .as_string()
            .expect("PASSWORD isn't set"),
        *config["DATABASE_HOST"].as_string().expect("HOST isn't set"),
        *config["DATABASE_PORT"].as_i64().expect("PORT isn't set"),
        *config["DATABASE_NAME"].as_string().expect("NAME isn't set"),
    );
    let config_data = YamlData {
        client_id: config["CLIENT_ID"]
            .as_string()
            .expect("CLIENT_ID isn't set")
            .to_string(),
        client_secret: config["CLIENT_SECRET"]
            .as_string()
            .expect("CLIENT_SECRET isn't set")
            .to_string(),
        redirect_url: config["REDIRECT_URL"]
            .as_string()
            .expect("REDIRECT_URL isn't set")
            .to_string(),
    };
    let connection = Data::new(Arc::new(Mutex::new(
        database_connection(database_url).expect("Database connection error"),
    )));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&connection))
            .app_data(Data::new(config_data.clone()))
            .service(index)
            .service(authorize)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))
    .expect("Cannot run application on port 8080")
    .run()
    .await
}
