
use diesel::{query_dsl::methods::SelectDsl, RunQueryDsl, SelectableHelper, result::Error};

use crate::models::{app_state::AppState, user::{NewUser, User}};

pub struct UsersRepository {}

impl UsersRepository {
    pub fn get_users_from_db(state: AppState) -> Result<Vec<User>, Error> {
        use crate::schema::users::dsl::*;
        let mut connection = state.pools.get().unwrap();
        let results = users
            .select(User::as_select())
            .load(&mut *connection);
        results
    }
    pub fn add_user_to_db(state: AppState, name: &str, surname: &str, age: &i32) -> User {
        use crate::schema::users;

        let mut connection = state.pools.get().unwrap();

        let new_user = NewUser {
            name, surname, age
        };

        diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut connection)
        .expect("error saving user")
    }
}