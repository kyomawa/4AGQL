use async_graphql::*;
use common::utils::send_graphql_request;
use serde_json::json;

use crate::schema::User;
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

    let _ = send_graphql_request::<serde_json::Value>(
        "http://auth-service:8080/api/auth/graphql",
        &payload,
        "deleteAuthByUserId",
    )
    .await?;

    Ok(())
}

// =============================================================================================================================

pub async fn delete_user_grades_request(user_id: &str) -> Result<()> {
    let grades_payload = json!({
        "query": r#"
            mutation DeleteGradesByUserId($userId: String!) {
                deleteGradesByUserId(userId: $userId) {
                    _id
                }
            }
        "#,
        "variables": { "userId": user_id }
    });
    let _ = send_graphql_request::<serde_json::Value>(
        "http://grades-service:8080/api/grades/graphql",
        &grades_payload,
        "deleteGradesByUserId",
    )
    .await?;

    Ok(())
}

// =============================================================================================================================

pub async fn remove_user_from_all_classes(user: &User, user_id: &str) -> Result<()> {
    for class_oid in user.class_ids.iter() {
        let remove_payload = json!({
            "query": r#"
                mutation RemoveStudent($classId: String!, $studentId: String!) {
                    removeStudent(classId: $classId, studentId: $studentId) {
                        _id
                    }
                }
            "#,
            "variables": {
                "classId": class_oid.to_hex(),
                "studentId": user_id,
            }
        });
        let _ = send_graphql_request::<serde_json::Value>(
            "http://classes-service:8080/api/classes/graphql",
            &remove_payload,
            "removeStudent",
        )
        .await?;
    }
    Ok(())
}

// =============================================================================================================================
