use async_graphql::*;
use common::jwt::jwt_utils::Claims;

// =============================================================================================================================

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> &str {
        "🟢 Auth Service is Alive"
    }

    // =============================================================================================================================

    async fn get_current_auth(&self, ctx: &Context<'_>) -> Result<Claims> {
        let claims = ctx
            .data_opt::<Claims>()
            .ok_or_else(|| Error::new("Unauthorized: token missing"))?;
        Ok(claims.clone())
    }
}

// =============================================================================================================================
