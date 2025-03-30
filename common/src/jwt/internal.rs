use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;

use crate::schemas::AuthRole;

use super::jwt_utils::Claims;

// =============================================================================================================================

pub static JWT_INTERNAL_SIGNATURE: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret_str = env::var("JWT_INTERNAL_SIGNATURE").expect("JWT_INTERNAL_SIGNATURE not set");
    secret_str.into_bytes()
});

// =============================================================================================================================

pub fn encode_internal_jwt() -> Result<String, String> {
    let claims = Claims {
        internal: true,
        exp: (Utc::now() + Duration::seconds(20)).timestamp(),
        user_id: "dummy".to_string(),
        role: AuthRole::Admin,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_INTERNAL_SIGNATURE.as_slice()),
    )
    .map_err(|e| e.to_string())
}

// =============================================================================================================================

pub fn decode_internal_jwt(token: &str) -> Result<Claims, String> {
    let signature = JWT_INTERNAL_SIGNATURE.as_slice();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(signature),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}

// =============================================================================================================================

pub fn authenticate_internal_request(claims: &Claims) -> Result<&Claims, String> {
    if claims.internal {
        Ok(claims)
    } else {
        Err("Forbidden: this call requires an internal token".to_string())
    }
}

// =============================================================================================================================
