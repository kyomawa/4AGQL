use actix_web::{post, web};
use async_graphql::EmptySubscription;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{mutation::MutationRoot, query::QueryRoot};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/users").service(graphql_handler);

    cfg.service(scope);
}

// =============================================================================================================================

#[post("/graphql")]
async fn graphql_handler(
    schema: web::Data<async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

// =============================================================================================================================
