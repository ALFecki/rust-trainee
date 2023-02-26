use crate::models::{NewUser, User};
use crate::schema::users;
use crate::schema::users::{email, name};
use diesel::pg::Pg;
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use diesel::{AsChangeset, Connection, Insertable, PgConnection, QueryDsl, Queryable};
use std::env;

pub fn database_connection() -> Result<PgConnection, String> {
    dotenv::dotenv().ok();

    match PgConnection::establish(&env::var("DATABASE_URL").expect("DATABASE_URL needs to be set"))
    {
        Ok(connection) => Ok(connection),
        Err(_) => Err("Cannot connect to database".to_string()),
    }
}

pub fn create_user(connection: &mut PgConnection, new_user: NewUser) -> User {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(connection)
        .expect("Cannot insert to table")
}
