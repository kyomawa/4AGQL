use async_graphql::Enum;
use serde::{Deserialize, Serialize};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Enum)]
pub enum AuthRole {
    User,
    Professor,
    Admin,
}

// =============================================================================================================================
