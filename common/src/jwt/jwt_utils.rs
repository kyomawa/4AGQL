use actix_web::http::header::{self, HeaderMap};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use crate::schemas::AuthRole;

use super::{external::decode_external_jwt, internal::decode_internal_jwt};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Claims {
    pub internal: bool,
    pub exp: i64,
    pub user_id: String,
    pub role: AuthRole,
}

// =============================================================================================================================

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<Claims> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_str| {
            auth_str
                .strip_prefix("Bearer ")
                .and_then(|token| match decode_internal_jwt(token) {
                    Ok(internal_claims) => Some(Claims {
                        internal: internal_claims.internal,
                        user_id: internal_claims.user_id,
                        role: internal_claims.role,
                        exp: internal_claims.exp,
                    }),
                    Err(_) => decode_external_jwt(token).ok(),
                })
        })
}

// =============================================================================================================================
