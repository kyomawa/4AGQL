use async_graphql::*;
use common::{
    jwt::{external::user_has_any_of_these_roles, jwt_utils::Claims},
    schemas::AuthRole,
};
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use validator::Validate;

use crate::request::check_student_exists;
use crate::schema::{Class, CreateClassRequest, UpdateClassRequest};

// =============================================================================================================================
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_class(&self, ctx: &Context<'_>, class: CreateClassRequest) -> Result<Class> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        class.validate()?;

        let creator_id = ObjectId::parse_str(&token.user_id)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");

        let mut new_class = Class {
            _id: None,
            name: class.name,
            user_ids: Vec::new(),
            creator_id,
        };

        let res = collection.insert_one(&new_class).await?;
        new_class._id = res.inserted_id.as_object_id();

        Ok(new_class)
    }

    // =============================================================================================================================

    async fn update_class_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
        class: UpdateClassRequest,
    ) -> Result<Class> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        class.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");
        let oid = ObjectId::parse_str(&id)?;

        if token.role == AuthRole::Professor {
            let existing_class = collection
                .find_one(doc! { "_id": oid.clone() })
                .await?
                .ok_or("No class with this id was found")?;
            if existing_class.creator_id.to_hex() != token.user_id {
                return Err(
                    "Unauthorized: A professor can only update a class they created".into(),
                );
            }
        }

        let update_doc = doc! {
            "$set": { "name": class.name }
        };

        let updated_class = collection
            .find_one_and_update(doc! { "_id": oid }, update_doc)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or("No class with this id was found")?;

        Ok(updated_class)
    }

    // =============================================================================================================================

    async fn delete_class_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Class> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");
        let oid = ObjectId::parse_str(&id)?;

        let class = collection
            .find_one(doc! { "_id": oid.clone() })
            .await?
            .ok_or("No class with this id was found")?;

        if !class.user_ids.is_empty() {
            return Err("Cannot delete a class with enrolled students".into());
        }

        if token.role == AuthRole::Professor && class.creator_id.to_hex() != token.user_id {
            return Err("Unauthorized: A professor can only delete a class they created".into());
        }

        let deleted_class = collection
            .find_one_and_delete(doc! { "_id": oid })
            .await?
            .ok_or("No class with this id was found")?;

        Ok(deleted_class)
    }

    // =============================================================================================================================

    async fn enroll_student(
        &self,
        ctx: &Context<'_>,
        class_id: String,
        student_id: String,
    ) -> Result<Class> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        check_student_exists(&student_id).await?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");
        let class_oid = ObjectId::parse_str(&class_id)?;
        let student_oid = ObjectId::parse_str(&student_id)?;

        if token.role == AuthRole::Professor {
            let existing_class = collection
                .find_one(doc! { "_id": class_oid.clone() })
                .await?
                .ok_or("No class with this id was found")?;
            if existing_class.creator_id.to_hex() != token.user_id {
                return Err(
                    "Unauthorized: A professor can only enroll students in classes they created"
                        .into(),
                );
            }
        }

        let update_doc = doc! {
            "$addToSet": { "user_ids": student_oid }
        };

        let updated_class = collection
            .find_one_and_update(doc! { "_id": class_oid }, update_doc)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or("No class with this id was found")?;

        Ok(updated_class)
    }

    // =============================================================================================================================

    async fn remove_student(
        &self,
        ctx: &Context<'_>,
        class_id: String,
        student_id: String,
    ) -> Result<Class> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        check_student_exists(&student_id).await?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");
        let class_oid = ObjectId::parse_str(&class_id)?;
        let student_oid = ObjectId::parse_str(&student_id)?;

        if token.role == AuthRole::Professor {
            let existing_class = collection
                .find_one(doc! { "_id": class_oid.clone() })
                .await?
                .ok_or("No class with this id was found")?;
            if existing_class.creator_id.to_hex() != token.user_id {
                return Err(
                    "Unauthorized: A professor can only remove students from classes they created"
                        .into(),
                );
            }
        }

        let update_doc = doc! {
            "$pull": { "user_ids": student_oid }
        };

        let updated_class = collection
            .find_one_and_update(doc! { "_id": class_oid }, update_doc)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or("No class with this id was found")?;

        Ok(updated_class)
    }
}
