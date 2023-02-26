use crate::schema::*;
use diesel::prelude::*;

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
        use crate::schema::users::dsl::*;
        use crate::schema::users::email;

        let user_in_db = users
            .filter(email.eq(&self.email))
            .limit(1)
            .load::<User>(connection)
            .expect("some");

        !user_in_db.is_empty()
    }
}
