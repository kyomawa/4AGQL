use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use std::fmt;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Enum)]
pub enum AuthRole {
    User,
    Professor,
    Admin,
}

impl fmt::Display for AuthRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AuthRole::User => "USER",
            AuthRole::Professor => "PROFESSOR",
            AuthRole::Admin => "ADMIN",
        };
        write!(f, "{}", s)
    }
}

// =============================================================================================================================
