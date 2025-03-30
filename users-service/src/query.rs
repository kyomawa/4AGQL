use async_graphql::*;
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

    async fn get_user_by_id(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = ObjectId::parse_str(&id)?;
        match collection.find_one(doc! {"_id": id}).await? {
            Some(user) => Ok(user),
            None => Err("Error retrieving user.".into()),
        }
    }

    async fn get_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let cursor: Cursor<User> = collection.find(doc! {}).await?;
        let users: Vec<User> = cursor.try_collect().await?;

        Ok(users)
    }
}

// =============================================================================================================================
