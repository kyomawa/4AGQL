use actix_web::{post, web};
use async_graphql::EmptySubscription;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use common::jwt::jwt_utils::get_token_from_request;

use crate::{mutation::MutationRoot, query::QueryRoot};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/grades").service(graphql_handler);

    cfg.service(scope);
}

// =============================================================================================================================

#[post("/graphql")]
async fn graphql_handler(
    schema: web::Data<async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>>,
    req: actix_web::HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    let mut request = gql_req.into_inner();
    if let Some(token) = get_token_from_request(&req) {
        request = request.data(token);
    }
    schema.execute(request).await.into()
}

// =============================================================================================================================
