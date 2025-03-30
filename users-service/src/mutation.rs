use async_graphql::*;
use common::{
    jwt::{
        external::user_has_any_of_these_roles, internal::authenticate_internal_request,
        jwt_utils::Claims,
    },
    schemas::AuthRole,
};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use validator::Validate;

use crate::schema::{CreateUserRequest, UpdateUserRequest, User};

// =============================================================================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, user: CreateUserRequest) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Forbidden: token invalid or missing")?;
        authenticate_internal_request(token)?;

        user.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let mut new_user = User {
            _id: None,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            pseudo: user.pseudo,
            class_ids: Vec::new(),
        };

        let res = collection.insert_one(&new_user).await?;
        let id = res.inserted_id.as_object_id().unwrap();

        new_user._id = Some(id);

        Ok(new_user)
    }

    // =============================================================================================================================

    async fn update_current_user(
        &self,
        ctx: &Context<'_>,
        user: UpdateUserRequest,
    ) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        user.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id = ObjectId::parse_str(&token.user_id)?;

        let mut set_doc = doc! {
            "first_name": user.first_name,
            "last_name": user.last_name,
            "email": user.email,
            "pseudo": user.pseudo,
        };

        if token.role == AuthRole::Admin {
            set_doc.insert("class_ids", user.class_ids);
        }

        let update_doc = doc! {
            "$set": set_doc,
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

    async fn update_user_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
        user: UpdateUserRequest,
    ) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Admin];

        user_has_any_of_these_roles(token, required_roles)?;

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
                "class_ids": user.class_ids,
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

    async fn delete_current_user(&self, ctx: &Context<'_>) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<User>("users");

        let id: ObjectId = ObjectId::parse_str(&token.user_id)?;

        match collection.find_one_and_delete(doc! {"_id": id}).await? {
            Some(user) => Ok(user),
            None => Err("No user with this is was found".into()),
        }
    }

    // =============================================================================================================================

    async fn delete_user_by_id(&self, ctx: &Context<'_>, id: String) -> Result<User> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Admin];

        user_has_any_of_these_roles(token, required_roles)?;

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
