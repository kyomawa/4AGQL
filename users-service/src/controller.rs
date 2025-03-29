use actix_web::{HttpResponse, get, post, web};
use async_graphql::{EmptySubscription, http::GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::{mutation::MutationRoot, query::QueryRoot};

// =============================================================================================================================

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/users")
        .service(graphql_handler)
        .service(playground);

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

#[get("/playground")]
async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            GraphQLPlaygroundConfig::new("/api/graphql"),
        ))
}
// =============================================================================================================================
