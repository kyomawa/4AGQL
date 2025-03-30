use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use once_cell::sync::Lazy;
use std::env;

use crate::schemas::AuthRole;

use super::jwt_utils::Claims;

// =============================================================================================================================

pub static JWT_EXTERNAL_SIGNATURE: Lazy<Vec<u8>> = Lazy::new(|| {
    let secret_str = env::var("JWT_EXTERNAL_SIGNATURE").expect("JWT_EXTERNAL_SIGNATURE not set");
    secret_str.into_bytes()
});

// =============================================================================================================================

pub fn encode_external_jwt(user_id: String, role: AuthRole) -> Result<String, String> {
    let signature = JWT_EXTERNAL_SIGNATURE.as_slice();
    let claims = Claims {
        internal: false,
        user_id,
        role,
        exp: (Utc::now() + Duration::minutes(120)).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(signature),
    )
    .map_err(|e| e.to_string())
}

// =============================================================================================================================

pub fn decode_external_jwt(token: &str) -> Result<Claims, String> {
    let signature = JWT_EXTERNAL_SIGNATURE.as_slice();
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(signature),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}

// =============================================================================================================================

pub fn get_authenticated_user(token: &Claims) -> &Claims {
    token
}

// =============================================================================================================================

pub fn user_has_any_of_these_roles<'a>(
    token: &'a Claims,
    roles: &[AuthRole],
) -> Result<&'a Claims, String> {
    if roles.contains(&token.role) {
        Ok(token)
    } else {
        Err("Access denied: insufficient role".to_string())
    }
}

// =============================================================================================================================
