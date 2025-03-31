use crate::schema::Class;
use async_graphql::*;
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
}
