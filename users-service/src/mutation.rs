use async_graphql::*;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};

use crate::model::{CreateUser, User};

// =============================================================================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUser) -> Result<User> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let new_user = User {
            _id: ObjectId::new(),
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
        };

        match collection.insert_one(&new_user).await {
            Ok(_) => Ok(new_user),
            Err(e) => Err(e.into()),
        }
    }

    // =============================================================================================================================

    async fn update_user(&self, ctx: &Context<'_>, id: String, input: CreateUser) -> Result<User> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = ObjectId::parse_str(&id)?;

        let update_doc = doc! {
            "$set": {
                "first_name": input.first_name,
                "last_name": input.last_name,
                "email": input.email
            }
        };

        match collection
            .find_one_and_update(doc! {"_id": id}, update_doc)
            .return_document(ReturnDocument::After)
            .await?
        {
            Some(user) => Ok(user),
            None => Err("No user with this id was found".into()),
        }
    }

    // =============================================================================================================================

    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = ObjectId::parse_str(&id)?;

        match collection.find_one_and_delete(doc! {"_id": id}).await? {
            Some(user) => Ok(user),
            None => Err("No user with this is was found".into()),
        }
    }
}

// =============================================================================================================================
