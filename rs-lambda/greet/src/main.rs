#![allow(non_snake_case)]

use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use chrono::Utc;

use serde_json;

use greet::schema::{ClientErrorMessage, Message, Request, Response};

async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<Response, Error> {
    let requestBodyText = event.payload.body;

    let resp: Response;

    match requestBodyText {
        Some(requestBody) => {
            let request: Request = serde_json::from_str(&requestBody)?;

            let message = Message {
                message: "Hello, World from cargo-lambda!!!".to_string(),
                client: format!("id={}|name={}", request.id, request.name),
                datetime: Utc::now().to_string(),
            };

            resp = Response {
                statusCode: 200,
                body: serde_json::to_string(&message)?,
            };
        }
        None => {
            let body = ClientErrorMessage {
                message: "No request body".to_string(),
                datetime: Utc::now().to_string(),
            };
            resp = Response {
                statusCode: 400,
                body: serde_json::to_string(&body)?,
            };
        }
    }
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
