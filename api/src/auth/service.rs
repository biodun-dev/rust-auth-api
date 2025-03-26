use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use crate::auth::handler::AuthPayload;
use crate::auth::jwt::generate_token;
use crate::db::user::{Entity as User, ActiveModel, Model, Column};

pub async fn register(db: DatabaseConnection, payload: AuthPayload) -> Result<String, DbErr> {
    let hashed = hash_password(&payload.password)
        .map_err(|e| DbErr::Custom(format!("Hashing error: {}", e)))?;

    let user = ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(payload.email),
        password_hash: Set(hashed),
        created_at: Set(Utc::now()),
    };

    User::insert(user).exec(&db).await?;
    Ok("User registered".to_string())
}

pub async fn login(db: DatabaseConnection, payload: AuthPayload) -> Result<String, DbErr> {
    let user = User::find()
        .filter(Column::Email.eq(payload.email.clone()))
        .one(&db)
        .await?
        .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;

    let is_valid = verify_password(&payload.password, &user.password_hash)
        .map_err(|e| DbErr::Custom(format!("Verification error: {}", e)))?;

    if is_valid {
        let token = generate_token(user.email)
            .map_err(|e| DbErr::Custom(format!("Token generation failed: {}", e)))?;
        Ok(token)
    } else {
        Err(DbErr::RecordNotFound("Invalid credentials".to_string()))
    }
}


fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}
