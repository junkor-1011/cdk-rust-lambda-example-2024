#![allow(non_snake_case)]

pub mod schema {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    pub struct Request {
        pub name: String,
        pub id: u32,
    }

    #[derive(Serialize)]
    pub struct Message {
        pub message: String,
        pub client: String,
        pub datetime: String,
    }

    #[derive(Serialize)]
    pub struct ClientErrorMessage {
        pub message: String,
        pub datetime: String,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub statusCode: i32,
        pub body: String,
    }
}
