mod context;
mod errors;

pub use bench_graphql::new_schema;
use bench_graphql::BenchqlSchema;
pub use context::{prepare_context, Context};
pub use errors::Error;

use actix_web::{web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use http::StatusCode;
use serde_json::json;

#[actix_web::get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json(&json!(
        {
            "status": "success"
        }
    ))
}

#[actix_web::route("/playground", method = "GET", method = "POST", method = "OPTIONS")]
pub async fn playground() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/api/graphql"),
        ))
}

#[actix_web::route("/graphql", method = "GET", method = "POST")]
pub async fn graphql_route(
    schema: web::Data<BenchqlSchema>,
    ctx: web::Data<Context>,
    request: GraphQLRequest,
    token: Option<BearerAuth>,
) -> Result<GraphQLResponse, Error> {
    let ctx: Context = ctx.as_ref().clone();
    let token = token.map(|t| t.token().to_string()); //not doing anything with auth now
    let context = ctx.as_authed_graphql_context(token);
    let req = request.into_inner().data(context);

    Ok(schema.execute(req).await.into())
}
