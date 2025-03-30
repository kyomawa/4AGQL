use async_graphql::*;
use serde_json::json;
// =============================================================================================================================

pub async fn delete_auth_by_user_id_request(user_id: &str) -> Result<()> {
    let query = r#"
      mutation DeleteAuthByUserId($userId: String!) {
          deleteAuthByUserId(userId: $userId) {
              userId
          }
      }
  "#;

    let variables = json!({
        "userId": user_id,
    });

    let payload = json!({
        "query": query,
        "variables": variables,
    });

    let _ = common::utils::send_graphql_request::<serde_json::Value>(
        "http://auth-service:8080/api/auth/graphql",
        &payload,
        "deleteAuthByUserId",
    )
    .await?;

    Ok(())
}

// =============================================================================================================================
