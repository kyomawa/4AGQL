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
