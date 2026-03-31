use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::db::DbPool;
use crate::error::{AppError, AppResult};
use super::models::{Claims, CreateUserRequest, UpdateUserRequest, User, UserDto};

pub struct AuthService;

impl AuthService {
    pub fn hash_password(password: &str, salt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", password, salt));
        hex::encode(hasher.finalize())
    }

    pub fn generate_salt() -> String {
        (0..32).map(|_| format!("{:02x}", rand::random::<u8>())).collect()
    }

    pub fn generate_token(user: &User, secret: &str, expiry_hours: i64) -> AppResult<String> {
        let now = Utc::now();
        let exp = (now + chrono::Duration::hours(expiry_hours)).timestamp() as usize;
        let iat = now.timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            user_id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub async fn authenticate(pool: &DbPool, username: &str, password: &str) -> AppResult<User> {
        let client = pool.get().await?;

        let row = client
            .query_opt(
                "SELECT id, username, email, password_hash, salt, role, is_active, first_name, last_name, created_at, updated_at
                 FROM users WHERE username = $1 AND is_active = true",
                &[&username],
            )
            .await?
            .ok_or_else(|| AppError::BadRequest("Invalid credentials".to_string()))?;

        let salt: String = row.get("salt");
        let stored_hash: String = row.get("password_hash");
        let computed_hash = Self::hash_password(password, &salt);

        if computed_hash != stored_hash {
            return Err(AppError::BadRequest("Invalid credentials".to_string()));
        }

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get("role"),
            is_active: row.get("is_active"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_user_by_id(pool: &DbPool, user_id: Uuid) -> AppResult<User> {
        let client = pool.get().await?;

        let row = client
            .query_opt(
                "SELECT id, username, email, role, is_active, first_name, last_name, created_at, updated_at
                 FROM users WHERE id = $1",
                &[&user_id],
            )
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get("role"),
            is_active: row.get("is_active"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn list_users(pool: &DbPool) -> AppResult<Vec<UserDto>> {
        let client = pool.get().await?;

        let rows = client
            .query(
                "SELECT id, username, email, role, first_name, last_name FROM users ORDER BY username",
                &[],
            )
            .await?;

        Ok(rows.iter().map(|row| UserDto {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get("role"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        }).collect())
    }

    pub async fn create_user(pool: &DbPool, req: &CreateUserRequest) -> AppResult<UserDto> {
        let client = pool.get().await?;

        let existing = client
            .query_opt("SELECT id FROM users WHERE username = $1 OR email = $2", &[&req.username, &req.email])
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict("Username or email already exists".to_string()));
        }

        let salt = Self::generate_salt();
        let password_hash = Self::hash_password(&req.password, &salt);
        let role = req.role.as_deref().unwrap_or("user");

        let row = client
            .query_one(
                "INSERT INTO users (username, email, password_hash, salt, role, first_name, last_name)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 RETURNING id, username, email, role, first_name, last_name",
                &[&req.username, &req.email, &password_hash, &salt, &role, &req.first_name, &req.last_name],
            )
            .await?;

        Ok(UserDto {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get("role"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        })
    }

    pub async fn update_user(pool: &DbPool, id: Uuid, req: &UpdateUserRequest) -> AppResult<UserDto> {
        let client = pool.get().await?;

        let row = client.query_opt(
            "UPDATE users SET
                email = COALESCE($1, email),
                role = COALESCE($2, role),
                first_name = COALESCE($3, first_name),
                last_name = COALESCE($4, last_name),
                is_active = COALESCE($5, is_active),
                updated_at = NOW()
             WHERE id = $6
             RETURNING id, username, email, role, first_name, last_name",
            &[&req.email, &req.role, &req.first_name, &req.last_name, &req.is_active, &id],
        ).await?.ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserDto {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            role: row.get("role"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
        })
    }

    pub async fn change_password(
        pool: &DbPool,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> AppResult<()> {
        let client = pool.get().await?;

        let row = client
            .query_opt(
                "SELECT password_hash, salt FROM users WHERE id = $1",
                &[&user_id],
            )
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        let salt: String = row.get("salt");
        let stored_hash: String = row.get("password_hash");
        let computed_hash = Self::hash_password(current_password, &salt);

        if computed_hash != stored_hash {
            return Err(AppError::BadRequest("Current password is incorrect".to_string()));
        }

        let new_salt = Self::generate_salt();
        let new_hash = Self::hash_password(new_password, &new_salt);

        client.execute(
            "UPDATE users SET password_hash = $1, salt = $2, updated_at = NOW() WHERE id = $3",
            &[&new_hash, &new_salt, &user_id],
        ).await?;

        Ok(())
    }
}
