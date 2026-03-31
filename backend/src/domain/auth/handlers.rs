use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension,
    Json,
};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::AppState;
use super::models::{
    ChangePasswordRequest, Claims, CreateUserRequest, LoginRequest, LoginResponse, UpdateUserRequest, UserDto,
};
use super::service::AuthService;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    if req.username.is_empty() || req.password.is_empty() {
        return Err(AppError::BadRequest("Username and password are required".to_string()));
    }

    let user = AuthService::authenticate(&state.pool, &req.username, &req.password).await?;
    let token = AuthService::generate_token(&user, &state.config.jwt_secret, state.config.jwt_expiry_hours)?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            token,
            user: UserDto {
                id: user.id,
                username: user.username,
                email: user.email,
                role: user.role,
                first_name: user.first_name,
                last_name: user.last_name,
            },
        }),
    ))
}

pub async fn logout() -> impl IntoResponse {
    Json(serde_json::json!({"message": "Logged out successfully"}))
}

pub async fn me(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = AuthService::get_user_by_id(&state.pool, claims.user_id).await?;
    Ok(Json(UserDto {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
        first_name: user.first_name,
        last_name: user.last_name,
    }))
}

pub async fn change_password(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<impl IntoResponse> {
    AuthService::change_password(
        &state.pool,
        claims.user_id,
        &req.current_password,
        &req.new_password,
    ).await?;

    Ok(Json(serde_json::json!({"message": "Password changed successfully"})))
}

pub async fn list_users(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    let users = AuthService::list_users(&state.pool).await?;
    Ok(Json(users))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<impl IntoResponse> {
    let user = AuthService::create_user(&state.pool, &req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<impl IntoResponse> {
    let user = AuthService::update_user(&state.pool, id, &req).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let client = state.pool.get().await?;
    let n = client
        .execute("DELETE FROM users WHERE id = $1", &[&id])
        .await?;

    if n == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
