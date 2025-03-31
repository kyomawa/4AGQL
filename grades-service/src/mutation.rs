use crate::{
    request::{check_professor_class_membership, check_user_exists},
    schema::{CreateGradeRequest, Grade, UpdateGradeRequest},
};
use async_graphql::*;
use common::{
    jwt::{
        external::user_has_any_of_these_roles, internal::authenticate_internal_request,
        jwt_utils::Claims,
    },
    schemas::AuthRole,
};
use futures_util::TryStreamExt;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use validator::Validate;

// =============================================================================================================================

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_grade(&self, ctx: &Context<'_>, grade: CreateGradeRequest) -> Result<Grade> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        check_user_exists(&grade.user_id).await?;

        if token.role == AuthRole::Professor {
            if token.user_id != grade.professor_id {
                return Err("Unauthorized: A professor can only create a grade with their own ID as professor_id".into());
            }
            check_professor_class_membership(&grade.professor_id, &grade.class_id).await?;
        }

        grade.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let user_oid = ObjectId::parse_str(&grade.user_id)?;
        let professor_oid = ObjectId::parse_str(&grade.professor_id)?;
        let class_oid = ObjectId::parse_str(&grade.class_id)?;

        let mut new_grade = Grade {
            _id: None,
            course: grade.course,
            note: grade.note,
            grade: grade.grade,
            user_id: user_oid,
            professor_id: professor_oid,
            class_id: class_oid,
        };

        let res = collection.insert_one(&new_grade).await?;

        new_grade._id = res.inserted_id.as_object_id();

        Ok(new_grade)
    }

    // =============================================================================================================================

    async fn update_grade_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
        grade: UpdateGradeRequest,
    ) -> Result<Grade> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        grade.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");
        let oid = ObjectId::parse_str(&id)?;

        let existing_grade = collection
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or("No grade with this id was found")?;

        if token.role == AuthRole::Professor
            && token.user_id != existing_grade.professor_id.to_hex()
        {
            return Err("Unauthorized: You can only update a grade that you created".into());
        }

        let update_doc = doc! {
            "$set": {
                "course": grade.course,
                "note": grade.note,
                "grade": grade.grade,
            }
        };

        let updated = collection
            .find_one_and_update(doc! {"_id": oid}, update_doc)
            .return_document(ReturnDocument::After)
            .await?
            .ok_or("No grade with this id was found")?;

        Ok(updated)
    }

    // =============================================================================================================================

    async fn delete_grade_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Grade> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");
        let oid = ObjectId::parse_str(&id)?;

        let existing_grade = collection
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or("No grade with this id was found")?;

        if token.role == AuthRole::Professor
            && token.user_id != existing_grade.professor_id.to_hex()
        {
            return Err("Unauthorized: You can only delete a grade that you created".into());
        }

        let deleted = collection
            .find_one_and_delete(doc! {"_id": oid})
            .await?
            .ok_or("No grade with this id was found")?;

        Ok(deleted)
    }

    // =============================================================================================================================

    async fn delete_grades_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> Result<Vec<Grade>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        authenticate_internal_request(token)?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let filter = doc! {
            "user_id": ObjectId::parse_str(&user_id)?
        };

        let cursor = collection.find(filter.clone()).await?;
        let grades: Vec<Grade> = cursor.try_collect().await?;

        collection.delete_many(filter).await?;

        Ok(grades)
    }
}

// =============================================================================================================================
