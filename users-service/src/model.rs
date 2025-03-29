use async_graphql::{InputObject, SimpleObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// =============================================================================================================================

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct User {
    pub _id: ObjectId,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, InputObject)]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

// =============================================================================================================================
