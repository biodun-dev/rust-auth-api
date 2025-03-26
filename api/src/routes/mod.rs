use axum::{Router, routing::post};
use crate::auth::handler::{register_user, login_user};

pub fn create_router() -> Router {
    Router::new()
        .route("/auth/register", post(register_user))
        .route("/auth/login", post(login_user))
}
