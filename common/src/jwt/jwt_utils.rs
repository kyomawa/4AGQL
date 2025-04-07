use actix_web::{HttpRequest, http::header};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

use super::{external::decode_external_jwt, internal::decode_internal_jwt};
use crate::schemas::AuthRole;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Claims {
    pub internal: bool,
    pub exp: i64,
    pub user_id: String,
    pub role: AuthRole,
}

// =============================================================================================================================

pub fn get_token_from_request(req: &HttpRequest) -> Option<Claims> {
    if let Some(cookie) = req.cookie("session_token") {
        let token = cookie.value();
        if let Ok(internal_claims) = decode_internal_jwt(token) {
            return Some(Claims {
                internal: internal_claims.internal,
                user_id: internal_claims.user_id,
                role: internal_claims.role,
                exp: internal_claims.exp,
            });
        } else if let Ok(external_claims) = decode_external_jwt(token) {
            return Some(external_claims);
        }
    }

    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_str| {
            auth_str.strip_prefix("Bearer ").and_then(|token| {
                if let Ok(internal_claims) = decode_internal_jwt(token) {
                    Some(Claims {
                        internal: internal_claims.internal,
                        user_id: internal_claims.user_id,
                        role: internal_claims.role,
                        exp: internal_claims.exp,
                    })
                } else {
                    decode_external_jwt(token).ok()
                }
            })
        })
}

// =============================================================================================================================
