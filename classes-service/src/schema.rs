use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Class {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    pub name: String,
    pub creator_id: ObjectId,

    #[serde(default)]
    pub user_ids: Vec<ObjectId>,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct CreateClassRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(length(min = 1, message = "Creator ID cannot be empty"))]
    pub creator_id: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct UpdateClassRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,
}

// =============================================================================================================================
