use crate::schema::{Class, ClassWithCreatorNames, GetUserFirstAndLastNameResponse};
use async_graphql::*;
use common::{jwt::jwt_utils::Claims, schemas::AuthRole, utils::send_graphql_request};
use futures_util::TryStreamExt;
use mongodb::{
    Database,
    bson::{doc, oid::ObjectId},
};
use serde_json::json;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "🟢 Classes Service is Alive"
    }

    // =============================================================================================================================

    async fn get_classes(
        &self,
        ctx: &Context<'_>,
        name: Option<String>,
    ) -> Result<Vec<ClassWithCreatorNames>> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");

        let mut filter = doc! {};
        if let Some(class_name) = name {
            filter.insert("name", class_name);
        }

        let cursor = collection.find(filter).await?;
        let classes: Vec<Class> = cursor.try_collect().await?;

        let mut classes_with_names = Vec::with_capacity(classes.len());

        for class in classes {
            let query = r#"
                query GetUserById($id: ID!) {
                    getUserById(id: $id) {
                        firstName
                        lastName
                    }
                }
            "#;

            let variables = json!({
                "id": class.creator_id.to_hex(),
            });

            let payload = json!({
                "query": query,
                "variables": variables,
            });

            let professor_names: GetUserFirstAndLastNameResponse = send_graphql_request(
                "http://users-service:8080/api/users/graphql",
                &payload,
                "getUserById",
            )
            .await?;

            let class_with_names = ClassWithCreatorNames {
                _id: class._id,
                name: class.name,
                creator_id: class.creator_id,
                creator_names: format!(
                    "{} {}",
                    professor_names.first_name, professor_names.last_name
                ),
                user_ids: class.user_ids,
            };

            classes_with_names.push(class_with_names);
        }

        Ok(classes_with_names)
    }

    // =============================================================================================================================

    async fn get_class_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<ClassWithCreatorNames> {
        let db = ctx.data_unchecked::<Database>();
        let collection = db.collection::<Class>("classes");
        let oid = ObjectId::parse_str(&id)?;

        let class = collection
            .find_one(doc! { "_id": oid })
            .await?
            .ok_or("No class with this id was found")?;

        let query = r#"
            query GetUserById($id: ID!) {
                getUserById(id: $id) {
                    firstName
                    lastName
                }
            }
        "#;

        let variables = json!({
            "id": class.creator_id.to_hex(),
        });

        let payload = json!({
            "query": query,
            "variables": variables,
        });

        let professor_names: GetUserFirstAndLastNameResponse = send_graphql_request(
            "http://users-service:8080/api/users/graphql",
            &payload,
            "getUserById",
        )
        .await?;

        let class = ClassWithCreatorNames {
            _id: class._id,
            name: class.name,
            creator_id: class.creator_id,
            creator_names: format!(
                "{} {}",
                professor_names.first_name, professor_names.last_name
            ),
            user_ids: class.user_ids,
        };

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
