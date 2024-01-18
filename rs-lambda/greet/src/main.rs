#![allow(non_snake_case)]

use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use chrono::Utc;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json;

use greet::schema::{ClientErrorMessage, Message, Request, Response};

async fn function_handler(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<Response, Error> {
    let request_body_text = event.payload.body;

    let request_body_text = match request_body_text {
        Some(text) => text,
        _ => {
            let body = ClientErrorMessage {
                message: "No request body".to_string(),
                datetime: Utc::now().to_string(),
            };
            let resp = Response {
                statusCode: 400,
                body: serde_json::to_string(&body)?,
            };
            return Ok(resp);
        }
    };

    let request: Result<Request, serde_json::Error> = serde_json::from_str(&request_body_text);

    let request = match request {
        Ok(obj) => obj,
        Err(err) => {
            println!("[DEBUG]parse error: {:?}", err);
            let body = ClientErrorMessage {
                message: "validation error".to_string(),
                datetime: Utc::now().to_string(),
            };
            let resp = Response {
                statusCode: 400,
                body: serde_json::to_string(&body)?,
            };
            return Ok(resp);
        }
    };

    let resp = {
        let message = Message {
            message: "Hello, World from cargo-lambda!!!".to_string(),
            client: format!("id={}|name={}", request.id, request.name),
            datetime: Utc::now().to_string(),
        };
        Response {
            statusCode: 200,
            body: serde_json::to_string(&message)?,
        }
    };
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
