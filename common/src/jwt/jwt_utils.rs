use actix_web::http::header::{self, HeaderMap};
use bson::oid::ObjectId;

use crate::schemas::AuthRole;

use super::{
    external::{ExternalClaims, decode_external_jwt},
    internal::decode_internal_jwt,
};

// =============================================================================================================================

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<ExternalClaims> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|auth_str| {
            auth_str
                .strip_prefix("Bearer ")
                .and_then(|token| match decode_internal_jwt(token) {
                    Ok(internal_claims) => Some(ExternalClaims {
                        user_id: ObjectId::new().to_hex(),
                        role: AuthRole::Admin,
                        exp: internal_claims.exp,
                    }),
                    Err(_) => decode_external_jwt(token).ok(),
                })
        })
}

// =============================================================================================================================
