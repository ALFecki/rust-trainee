use crate::schema::users::email;
use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    name: String,
    email: String,
}

impl User {
    pub fn is_exists(&self, connection: &mut PgConnection) -> bool {
        use crate::schema::users::dsl::users;

        let user_in_db = users
            .filter(email.eq(&self.email))
            .limit(1)
            .load::<User>(connection)
            .expect("some");

        !user_in_db.is_empty()
    }
}
