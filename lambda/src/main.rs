use std::env::set_var;
use axum::{Json, Router};
use axum::routing::get;
use lambda_http::{run, tracing, Error};
use lambda_http::tracing::{error, info, warn, debug};
use serde_json::{json, Value};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler() -> Json<Value> {
    // Extract some useful information from the request

    debug!("debug");
    info!("log");
    warn!("worn");
    error!("error");

    Json(json!({"msg": "return"}))
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(function_handler));

    run(app).await
}
