use crate::schema::Class;
use async_graphql::*;
use common::{jwt::jwt_utils::Claims, schemas::AuthRole};
use futures_util::TryStreamExt;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "🟢 Classes Service is Alive"
    }

    // =============================================================================================================================

    async fn get_classes(&self, ctx: &Context<'_>, name: Option<String>) -> Result<Vec<Class>> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");

        let mut filter = doc! {};

        if let Some(class_name) = name {
            filter.insert("name", class_name);
        }

        let cursor = collection.find(filter).await?;
        let classes = cursor.try_collect().await?;
        Ok(classes)
    }

    // =============================================================================================================================

    async fn get_class_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Class> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");
        let oid = ObjectId::parse_str(&id)?;

        let class = collection
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or("No class with this id was found")?;
        Ok(class)
    }

    // =============================================================================================================================

    async fn get_classes_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> Result<Vec<Class>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let requested_user_id = ObjectId::parse_str(&user_id)?;

        let filter = match token.role {
            AuthRole::Admin => {
                doc! {
                    "$or": [
                        { "creator_id": requested_user_id },
                        { "user_ids": { "$in": [requested_user_id] } }
                    ]
                }
            }
            AuthRole::User => {
                let connected_user = ObjectId::parse_str(&token.user_id)?;
                if requested_user_id != connected_user {
                    return Err("Unauthorized: You can only view your own classes".into());
                }
                doc! { "user_ids": { "$in": [requested_user_id] } }
            }
            AuthRole::Professor => {
                let connected_user = ObjectId::parse_str(&token.user_id)?;
                if requested_user_id != connected_user {
                    return Err("Unauthorized: You can only view your own classes".into());
                }
                doc! { "creator_id": connected_user }
            }
        };

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");

        let cursor = collection.find(filter).await?;
        let classes = cursor.try_collect().await?;

        Ok(classes)
    }

    // =============================================================================================================================
}
