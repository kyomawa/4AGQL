use async_graphql::*;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use validator::Validate;

use crate::schema::{CreateUser, User};

// =============================================================================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, user: CreateUser) -> Result<User> {
        user.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let mut new_user = User {
            _id: None,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            pseudo: user.pseudo,
            class_id: None,
        };

        let res = collection.insert_one(&new_user).await?;
        let id = res.inserted_id.as_object_id().unwrap();

        new_user._id = Some(id);

        Ok(new_user)
    }

    // =============================================================================================================================

    async fn update_user(&self, ctx: &Context<'_>, id: String, user: CreateUser) -> Result<User> {
        user.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = ObjectId::parse_str(&id)?;

        let update_doc = doc! {
            "$set": {
                "first_name": user.first_name,
                "last_name": user.last_name,
                "email": user.email,
                "pseudo": user.pseudo,
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
