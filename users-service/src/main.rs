use actix_web::{App, HttpServer, web};
use async_graphql::Schema;
use controller::config;
use db::init_db;
use mutation::MutationRoot;
use query::QueryRoot;

mod controller;
mod db;
mod model;
mod mutation;
mod query;

// =============================================================================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let db = init_db().await.expect("❌ Failed to connect to database");

    let graphql_schema = Schema::build(QueryRoot, MutationRoot, async_graphql::EmptySubscription)
        .data(db.clone())
        .finish();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(graphql_schema.clone()))
            .configure(config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

// =============================================================================================================================
