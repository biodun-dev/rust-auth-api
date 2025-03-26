use axum::{Extension, Json};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use axum::http::StatusCode;
use crate::auth::service::{register, login};
use utoipa::{ToSchema};

#[derive(Deserialize, ToSchema)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = AuthPayload,
    responses(
        (status = 201, description = "User created successfully", body = String),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn register_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<String>, StatusCode> {
    register(db, payload)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "Login successful", body = String),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<String>, StatusCode> {
    login(db, payload)
        .await
        .map(Json)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
