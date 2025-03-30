use async_graphql::{InputObject, SimpleObject};
use common::utils::{LETTERS_REGEX, trim_lowercase};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,

    pub first_name: String,
    pub last_name: String,
    pub pseudo: String,
    pub email: String,

    #[serde(default)]
    pub class_ids: Vec<ObjectId>,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct CreateUserRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 25,
        message = "First name must be between 2 and 25 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "First name contains invalid characters"
    ))]
    pub first_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 35,
        message = "Last name must be between 2 and 35 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "Last name contains invalid characters"
    ))]
    pub last_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 20,
        message = "Pseudo must be between 2 and 20 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "Pseudo contains invalid characters"
    ))]
    pub pseudo: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
}

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, InputObject, Validate)]
pub struct UpdateUserRequest {
    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 25,
        message = "First name must be between 2 and 25 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "First name contains invalid characters"
    ))]
    pub first_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 35,
        message = "Last name must be between 2 and 35 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "Last name contains invalid characters"
    ))]
    pub last_name: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(length(
        min = 2,
        max = 20,
        message = "Pseudo must be between 2 and 20 characters"
    ))]
    #[validate(regex(
        path = "*LETTERS_REGEX",
        message = "Pseudo contains invalid characters"
    ))]
    pub pseudo: String,

    #[serde(deserialize_with = "trim_lowercase")]
    #[validate(email(message = "Email must be valid"))]
    pub email: String,

    #[serde(default)]
    pub class_ids: Vec<ObjectId>,
}

// =============================================================================================================================
