use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub age: i32,
}

#[derive(Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub surname: String,
    pub age: i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub name: &'a str,    
    pub surname: &'a str,
    pub age: &'a i32,
}