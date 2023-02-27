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

pub fn create_user(connection: &mut PgConnection, new_user: NewUser) -> Result<User, &str> {
    use crate::schema::users::dsl::*;

    match diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(connection)
    {
        Ok(user) => Ok(user),
        Err(_) => Err("Error insert into table"),
    }
}

pub fn select_user(connection: &mut PgConnection, email: String) -> Option<User> {
    return match users::table
        .filter(users::email.eq(email))
        .limit(1)
        .load::<User>(connection)
    {
        Ok(user) => match user.is_empty() {
            true => None,
            false => Some(user[0].clone()),
        },
        Err(_) => None,
    };
}
