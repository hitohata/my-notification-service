use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use axum::Router;
use axum::routing::get;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use lambda_runtime::tracing::{error, info, warn, debug};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    // Extract some useful information from the request

    debug!("debug");
    info!("log");
    warn!("worn");
    error!("error");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();


    let app = Router::new()
        .route("/", get(function_handler));

    run(app).await
}
