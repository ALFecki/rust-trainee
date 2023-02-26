use crate::dbcontroller::select_user;
use crate::schema::*;
use diesel::prelude::*;
use std::borrow::Borrow;

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    email: String,
}

impl NewUser {
    pub fn new(email: String) -> Self {
        Self {
            name: {
                let email_cpy = email.clone();
                let vec: Vec<&str> = email_cpy.split('@').collect();
                vec[0].to_string()
            },
            email,
        }
    }

    pub fn is_exists(&self, connection: &mut PgConnection) -> bool {
        let user_in_db = select_user(connection, self.email.clone());
        user_in_db.is_some()
    }
}
