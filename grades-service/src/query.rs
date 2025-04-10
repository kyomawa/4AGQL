use async_graphql::*;
use common::{jwt::jwt_utils::Claims, schemas::AuthRole, utils::send_graphql_request};
use futures_util::TryStreamExt;
use mongodb::{
    Cursor, Database,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;

use crate::schema::{
    GetClassNameResponse, GetUserFirstAndLastNameResponse, Grade, GradeWithAllNames,
};

// =============================================================================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "🟢 Grades Service is Alive"
    }

    // =============================================================================================================================

    async fn get_grades(
        &self,
        ctx: &Context<'_>,
        course: Option<String>,
    ) -> Result<Vec<GradeWithAllNames>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let mut filter = doc! {};

        if let Some(course_name) = course {
            filter.insert("course", course_name);
        }

        match token.role {
            AuthRole::Admin => {}
            AuthRole::Professor => {
                let professor_oid = ObjectId::parse_str(&token.user_id)?;
                filter.insert("professor_id", professor_oid);
            }
            AuthRole::User => {
                let user_oid = ObjectId::parse_str(&token.user_id)?;
                filter.insert("user_id", user_oid);
            }
        }

        let cursor: Cursor<Grade> = collection.find(filter).await?;
        let grades: Vec<Grade> = cursor.try_collect().await?;

        let mut enriched_grades = Vec::with_capacity(grades.len());
        for grade in grades {
            enriched_grades.push(enrich_grade(grade).await?);
        }

        Ok(enriched_grades)
    }

    // =============================================================================================================================

    async fn get_grade_by_id(&self, ctx: &Context<'_>, id: String) -> Result<GradeWithAllNames> {
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

        match token.role {
            AuthRole::Admin => {}
            AuthRole::Professor => {
                let professor_oid = ObjectId::parse_str(&token.user_id)?;
                if grade.professor_id != professor_oid {
                    return Err(
                        "Unauthorized: You can only view grades from your own classes".into(),
                    );
                }
            }
            AuthRole::User => {
                let user_oid = ObjectId::parse_str(&token.user_id)?;
                if grade.user_id != user_oid {
                    return Err(
                        "Unauthorized: You do not have permission to view this grade".into(),
                    );
                }
            }
        }

        enrich_grade(grade).await
    }

    // =============================================================================================================================

    async fn get_grades_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> Result<Vec<GradeWithAllNames>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let filter = match token.role {
            AuthRole::Admin => {
                doc! {
                    "$or": [
                        { "user_id": ObjectId::parse_str(&user_id)? },
                        { "professor_id": ObjectId::parse_str(&user_id)? }
                    ]
                }
            }
            AuthRole::Professor => {
                let professor_oid = ObjectId::parse_str(&token.user_id)?;
                if professor_oid != ObjectId::parse_str(&user_id)? {
                    return Err("Unauthorized: You can only view your own grades".into());
                }
                doc! { "professor_id": professor_oid }
            }
            AuthRole::User => {
                let user_oid = ObjectId::parse_str(&token.user_id)?;
                if user_oid != ObjectId::parse_str(&user_id)? {
                    return Err("Unauthorized: You can only view your own grades".into());
                }
                doc! { "user_id": user_oid }
            }
        };

        let cursor: Cursor<Grade> = collection.find(filter).await?;
        let grades: Vec<Grade> = cursor.try_collect().await?;

        let mut enriched_grades = Vec::with_capacity(grades.len());
        for grade in grades {
            enriched_grades.push(enrich_grade(grade).await?);
        }
        Ok(enriched_grades)
    }

    // =============================================================================================================================

    async fn get_grades_by_class_id(
        &self,
        ctx: &Context<'_>,
        class_id: String,
    ) -> Result<Vec<GradeWithAllNames>> {
        let token = ctx
            .data_opt::<Claims>()
            .ok_or("Unauthorized: token missing or invalid")?;

        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Grade>("grades");

        let class_oid = ObjectId::parse_str(&class_id)?;

        let filter = match token.role {
            AuthRole::Admin => {
                doc! { "class_id": class_oid }
            }
            AuthRole::Professor => {
                let professor_oid = ObjectId::parse_str(&token.user_id)?;
                doc! { "class_id": class_oid, "professor_id": professor_oid }
            }
            AuthRole::User => {
                let user_oid = ObjectId::parse_str(&token.user_id)?;
                doc! { "class_id": class_oid, "user_id": user_oid }
            }
        };

        let cursor: Cursor<Grade> = collection.find(filter).await?;
        let grades: Vec<Grade> = cursor.try_collect().await?;

        let mut enriched_grades = Vec::with_capacity(grades.len());
        for grade in grades {
            enriched_grades.push(enrich_grade(grade).await?);
        }
        Ok(enriched_grades)
    }
}

// =============================================================================================================================

async fn enrich_grade(grade: Grade) -> Result<GradeWithAllNames> {
    let query_user = r#"
        query GetUserById($id: ID!) {
            getUserById(id: $id) {
                firstName
                lastName
            }
        }
    "#;

    let user_variables = json!({ "id": grade.user_id.to_hex() });
    let user_payload = json!({
        "query": query_user,
        "variables": user_variables,
    });

    let user_names: GetUserFirstAndLastNameResponse = match send_graphql_request(
        "http://users-service:8080/api/users/graphql",
        &user_payload,
        "getUserById",
    )
    .await
    {
        Ok(names) => names,
        Err(_) => GetUserFirstAndLastNameResponse {
            first_name: "Utilisateur".to_string(),
            last_name: "Supprimé".to_string(),
        },
    };

    let professor_variables = json!({ "id": grade.professor_id.to_hex() });
    let professor_payload = json!({
        "query": query_user,
        "variables": professor_variables,
    });

    let professor_names: GetUserFirstAndLastNameResponse = match send_graphql_request(
        "http://users-service:8080/api/users/graphql",
        &professor_payload,
        "getUserById",
    )
    .await
    {
        Ok(names) => names,
        Err(_) => GetUserFirstAndLastNameResponse {
            first_name: "Utilisateur".to_string(),
            last_name: "Supprimé".to_string(),
        },
    };

    let query_class = r#"
        query GetClassById($id: ID!) {
            getClassById(id: $id) {
                name
            }
        }
    "#;
    let class_variables = json!({ "id": grade.class_id.to_hex() });
    let class_payload = json!({
        "query": query_class,
        "variables": class_variables,
    });

    let class_name: GetClassNameResponse = match send_graphql_request(
        "http://classes-service:8080/api/classes/graphql",
        &class_payload,
        "getClassById",
    )
    .await
    {
        Ok(names) => names,
        Err(_) => GetClassNameResponse {
            name: "Classe Supprimé".to_string(),
        },
    };

    let enriched = GradeWithAllNames {
        _id: grade._id,
        course: grade.course,
        note: grade.note,
        grade: grade.grade,
        user_id: grade.user_id,
        user_names: format!("{} {}", user_names.first_name, user_names.last_name),
        professor_names: format!(
            "{} {}",
            professor_names.first_name, professor_names.last_name
        ),
        professor_id: grade.professor_id,
        class_id: grade.class_id,
        class_name: class_name.name,
    };

    Ok(enriched)
}

// =============================================================================================================================
