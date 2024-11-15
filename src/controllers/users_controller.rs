use crate::{
    models::{
        app_state::AppState,
        messages::{ResponseType, WebResponse},
        user::{User, UserPayload},
    },
    repositories::users_repository::UsersRepository,
    services::users_service::UsersService,
};
use axum::{
    extract::{Json, Path, State},
    routing::{get, post},
    Router,
};

struct UsersController;

impl UsersController {
    async fn index(State(state): State<AppState>) -> Json<WebResponse<Vec<User>>> {
        let users = UsersRepository::get_users_from_db(state);
        let response = match users {
            Ok(u) => WebResponse::<Vec<User>>::new(
                ResponseType::Data,
                "Getting users ended successfully".to_owned(),
                200,
                Some(u),
            ),
            Err(_) => WebResponse::new(
                ResponseType::Error,
                "Failed to get users".to_owned(),
                500,
                None,
            ),
        };
        Json(response)
    }
    async fn get_user(Path(_user_id): Path<String>) {
        todo!("returns target user");
    }
    async fn add_user(
        State(state): State<AppState>,
        payload: Json<UserPayload>,
    ) -> Json<WebResponse<String>> {
        let result = UsersService::add_user(state, payload).await;
        let data = match result {
            Ok(_) => WebResponse::new(
                ResponseType::Success,
                "User added successfully".to_owned(),
                200,
                None,
            ),
            Err(error) => WebResponse::new(ResponseType::Error, error.message, error.code, None),
        };
        Json(data)
    }
}

pub fn configure_paths() -> Router<AppState> {
    Router::new()
        .route("/", get(UsersController::index))
        .route("/addUser", post(UsersController::add_user))
        .route("/:user_id", get(UsersController::get_user))
}
