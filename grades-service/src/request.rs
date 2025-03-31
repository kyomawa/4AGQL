use async_graphql::Result;
use common::utils::send_graphql_request;
use serde_json::json;

// =============================================================================================================================

pub async fn check_user_exists(user_id: &str) -> Result<()> {
    let query = r#"
        query GetUserById($id: String!) {
            getUserById(id: $id) {
                id
            }
        }
    "#;

    let variables = json!({ "id": user_id });
    let payload = json!({
        "query": query,
        "variables": variables,
    });

    let user: serde_json::Value = send_graphql_request(
        "http://users-service:8080/api/users/graphql",
        &payload,
        "getUserById",
    )
    .await?;

    if user.is_null() {
        return Err("User not found".into());
    }
    Ok(())
}

// =============================================================================================================================

pub async fn check_professor_class_membership(
    professor_id: &str,
    grade_class_id: &str,
) -> Result<()> {
    let query = r#"
        query GetUserById($id: String!) {
            getUserById(id: $id) {
                id
                classIds
            }
        }
    "#;

    let variables = json!({ "id": professor_id });
    let payload = json!({
        "query": query,
        "variables": variables,
    });

    let professor: serde_json::Value = send_graphql_request(
        "http://users-service:8080/api/users/graphql",
        &payload,
        "getUserById",
    )
    .await?;

    if professor.is_null() {
        return Err("Professor not found".into());
    }

    let class_ids = professor
        .get("classIds")
        .ok_or("Professor classIds not found")?
        .as_array()
        .ok_or("Professor classIds is not an array")?;

    let mut found = false;
    for class_id in class_ids {
        if let Some(cid) = class_id.as_str() {
            if cid == grade_class_id {
                found = true;
                break;
            }
        }
    }
    if !found {
        return Err("Unauthorized: Grade class_id is not part of professor's classes".into());
    }
    Ok(())
}

// =============================================================================================================================
