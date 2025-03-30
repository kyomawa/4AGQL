use async_graphql::*;
use bcrypt::{DEFAULT_COST, hash};
use common::{schemas::AuthRole, utils::send_graphql_request};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;
use validator::Validate;

use crate::schema::{Auth, CreateUserInternalResponse, RegisterRequest};

// =============================================================================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register(&self, ctx: &Context<'_>, user: RegisterRequest) -> Result<Auth> {
        user.validate()?;

        let query = r#"
            mutation CreateUser($user: CreateUserRequest!) {
                createUser(user: $user) {
                    id
                    firstName
                    lastName
                    pseudo
                    email
                }
            }
        "#;

        let variables = json!({
            "user": {
                "firstName": user.first_name,
                "lastName": user.last_name,
                "pseudo": user.pseudo,
                "email": user.email,
            }
        });

        let payload = json!({
            "query": query,
            "variables": variables,
        });

        let created_user: CreateUserInternalResponse = send_graphql_request(
            "http://users-service:8080/api/users/graphql",
            &payload,
            "createUser",
        )
        .await?;

        let id = ObjectId::parse_str(&created_user.id)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Auth>("auth");

        let hashed_password = hash(&user.password, DEFAULT_COST)?;

        let mut credential = Auth {
            _id: None,
            password: hashed_password,
            role: AuthRole::User,
            user_id: id,
        };

        let result = collection.insert_one(&credential).await?;

        credential._id = result.inserted_id.as_object_id();

        Ok(credential)
    }
}

// =============================================================================================================================
