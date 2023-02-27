use crate::dbcontroller::select_user;
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
    pub fn new(email: String) -> Self {
        Self { email }
    }

    pub fn is_exists(&self, connection: &mut PgConnection) -> bool {
        let user_in_db = select_user(connection, self.email.clone());
        user_in_db.is_some()
    }
}
