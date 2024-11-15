use axum::Json;
use crate::{models::{app_state::AppState, user::{User, UserPayload}, error::Error as WebError}, repositories::users_repository::UsersRepository};

pub struct UsersService {}

impl UsersService {
    pub async fn add_user(state: AppState, payload: Json<UserPayload>) -> Result<User, WebError> {
        let UserPayload { name, surname, age} = payload.0;
        if name.len() == 0 || surname.len() == 0 || age <= 0 {
            return Err(WebError {
                code: 403,
                message: "invalid payload".to_owned()
            })
        }
        let result = UsersRepository::add_user_to_db(state, name.as_str(), surname.as_str(), &age);
        Ok(result)
    }
}
