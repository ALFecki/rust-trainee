use crate::models::{NewUser, User};
use crate::schema::users;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::{Connection, PgConnection, QueryDsl};

pub fn database_connection(database_url: String) -> Result<PgConnection, &'static str> {
    match PgConnection::establish(database_url.as_str()) {
        Ok(connection) => Ok(connection),
        Err(_) => Err("Cannot connect to database"),
    }
}

pub fn create_user(connection: &mut PgConnection, new_user: NewUser) -> Result<User, String> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(connection)
        .map_err(|e| e.to_string())
}

pub fn select_user(connection: &mut PgConnection, email: &str) -> Result<Option<User>, String> {
    users::table
        .filter(users::email.eq(email.to_string()))
        .limit(1)
        .load::<User>(connection)
        .map(|result| {
            if !result.is_empty() {
                Some(result[0].clone())
            } else {
                None
            }
        })
        .map_err(|e| e.to_string())
}
