use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    email: String,
}

impl NewUser {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}
