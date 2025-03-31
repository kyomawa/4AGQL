use async_graphql::*;
use common::{jwt::jwt_utils::Claims, schemas::AuthRole};
use futures_util::TryStreamExt;
use mongodb::{
    Cursor, Database,
    bson::{doc, oid::ObjectId},
};

use crate::schema::Grade;

// =============================================================================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "🟢 Grades Service is Alive"
    }

    // =============================================================================================================================

    async fn get_grades(&self, ctx: &Context<'_>, course: Option<String>) -> Result<Vec<Grade>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let mut filter = doc! {};

        if let Some(course_name) = course {
            filter.insert("course", course_name);
        }

        if token.role != AuthRole::Professor && token.role != AuthRole::Admin {
            let user_oid = ObjectId::parse_str(&token.user_id)?;
            filter.insert("user_id", user_oid);
        }

        let cursor: Cursor<Grade> = collection.find(filter).await?;
        let grades: Vec<Grade> = cursor.try_collect().await?;
        Ok(grades)
    }

    // =============================================================================================================================

    async fn get_grade_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Grade> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");
        let oid = ObjectId::parse_str(&id)?;

        let grade = collection
            .find_one(doc! {"_id": oid})
            .await?
            .ok_or("No grade with this id was found")?;

        if token.role != AuthRole::Professor && token.role != AuthRole::Admin {
            let token_oid = ObjectId::parse_str(&token.user_id)?;
            if grade.user_id != token_oid {
                return Err("Unauthorized: You do not have permission to view this grade".into());
            }
        }

        Ok(grade)
    }
}

// =============================================================================================================================
