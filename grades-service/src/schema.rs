use async_graphql::{InputObject, SimpleObject};
use common::utils::{trim, trim_lowercase};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Grade {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    pub course: String,
    pub note: String,
    pub grade: i32,
    pub user_id: ObjectId,
    pub professor_id: ObjectId,
    pub class_id: ObjectId,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct CreateGradeRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 1,
        max = 100,
        message = "Course name must be between 1 and 100 characters"
    ))]
    pub course: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 10,
        max = 250,
        message = "A note must be between 10 and 250 characters"
    ))]
    pub note: String,

    #[validate(range(min = 0, max = 100, message = "Grade must be between 0 and 100"))]
    pub grade: i32,

    #[validate(length(min = 1, message = "User ID cannot be empty"))]
    pub user_id: String,

    #[validate(length(min = 1, message = "User ID cannot be empty"))]
    pub professor_id: String,

    #[validate(length(min = 1, message = "User ID cannot be empty"))]
    pub class_id: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct UpdateGradeRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 1,
        max = 100,
        message = "Course name must be between 1 and 100 characters"
    ))]
    pub course: String,

    #[serde(deserialize_with = "trim")]
    #[validate(length(
        min = 10,
        max = 250,
        message = "A note must be between 10 and 250 characters"
    ))]
    pub note: String,

    #[validate(range(min = 0, max = 100, message = "Grade must be between 0 and 100"))]
    pub grade: i32,
}

// =============================================================================================================================
