use crate::{
    request::check_user_exists,
    schema::{CreateGradeRequest, Grade, UpdateGradeRequest},
};
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

        grade.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let user_oid = ObjectId::parse_str(&grade.user_id)?;

        let mut new_grade = Grade {
            _id: None,
            course: grade.course,
            grade: grade.grade,
            user_id: user_oid,
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
        input: UpdateGradeRequest,
    ) -> Result<Grade> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;
        let required_roles = &[AuthRole::Professor, AuthRole::Admin];
        user_has_any_of_these_roles(token, required_roles)?;

        input.validate()?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");
        let oid = ObjectId::parse_str(&id)?;

        let update_doc = doc! {
            "$set": {
                "course": input.course,
                "grade": input.grade,
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

        let deleted = collection
            .find_one_and_delete(doc! {"_id": oid})
            .await?
            .ok_or("No grade with this id was found")?;

        Ok(deleted)
    }
}

// =============================================================================================================================
