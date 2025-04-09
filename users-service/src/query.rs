use async_graphql::*;
use common::{
    jwt::{external::user_has_any_of_these_roles, jwt_utils::Claims},
    schemas::AuthRole,
};
use futures_util::TryStreamExt;
use mongodb::{
    Cursor, Database,
    bson::{doc, oid::ObjectId},
};

use crate::schema::User;

// =============================================================================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "🟢 Users Service is Alive"
    }

    // =============================================================================================================================

    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Admin];

        user_has_any_of_these_roles(token, required_roles)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let cursor: Cursor<User> = collection.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;

        Ok(users)
    }

    // =============================================================================================================================

    async fn get_users_by_class_id(
        &self,
        ctx: &Context<'_>,
        class_id: String,
    ) -> Result<Vec<User>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Admin, AuthRole::Professor];

        user_has_any_of_these_roles(token, required_roles)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let cursor: Cursor<User> = collection
            .find(doc! {"class_id": ObjectId::parse_str(&class_id)?})
            .await?;
        let users: Vec<User> = cursor.try_collect().await?;

        Ok(users)
    }

    // =============================================================================================================================

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Admin, AuthRole::Professor];

        user_has_any_of_these_roles(token, required_roles)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = ObjectId::parse_str(&id)?;
        match collection.find_one(doc! {"_id": id}).await? {
            Some(user) => Ok(user),
            None => Err("No user with this id was found.".into()),
        }
    }

    // =============================================================================================================================

    async fn get_user_by_email_or_pseudo(
        &self,
        ctx: &Context<'_>,
        credential: String,
    ) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Admin];

        user_has_any_of_these_roles(token, required_roles)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let credential = credential.to_lowercase();

        let filter = doc! {
            "$or": [
                { "email": credential.clone() },
                { "pseudo": credential }
            ]
        };

        match collection.find_one(filter).await? {
            Some(user) => Ok(user),
            None => Err("No user with this email or pseudo was found.".into()),
        }
    }

    // =============================================================================================================================

    async fn get_current_user(&self, ctx: &Context<'_>) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = token.user_id.clone();
        match collection
            .find_one(doc! {"_id": ObjectId::parse_str(&id)?})
            .await?
        {
            Some(user) => Ok(user),
            None => Err("Error retrieving user.".into()),
        }
    }
}

// =============================================================================================================================
