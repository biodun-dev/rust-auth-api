use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize};
use std::env;
use chrono::{Utc, Duration};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token(email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: email,
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
