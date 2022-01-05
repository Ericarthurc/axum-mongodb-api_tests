use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let api_routes = Router::new().route("/", get(handler));

    let app = Router::new()
        .fallback(get(handler_404))
        .nest("/api", api_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

async fn handler() -> Result<impl IntoResponse, AppError> {
    // let db = connect_mongo().await?;
    // let typed_collection = db.collection::<Book>("books");

    let client = connect_mongo().await?;

    // let collection = client.database("ok").collection("ok");

    let db = client.database("rusty");
    let collection = db.collection::<Book>("ok");

    let handle = tokio::task::spawn(async move {
        for collection_name in db.list_collection_names(None).await {
            println!("{:#?}", collection_name);
        }

        let books = vec![
            Book {
                title: "The Grapes of Wrath".to_string(),
                author: "John Steinbeck".to_string(),
            },
            Book {
                title: "To Kill a Mockingbird".to_string(),
                author: "Harper Lee".to_string(),
            },
        ];
        collection.insert_many(books, None).await
    });

    tokio::time::timeout(Duration::from_secs(5), handle).await???;

    Ok((StatusCode::FOUND, "ROOT!"))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

enum AppError {
    Mongo(mongodb::error::Error),
    Elapsed(tower::timeout::error::Elapsed),
    Tokio(TokioError),
}

enum TokioError {
    Elapsed(tokio::time::error::Elapsed),
    JoinError(tokio::task::JoinError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::Mongo(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            AppError::Elapsed(error) => (StatusCode::UNPROCESSABLE_ENTITY, error.to_string()),
            AppError::Tokio(TokioError::Elapsed(error)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, error.to_string())
            }
            AppError::Tokio(TokioError::JoinError(error)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, error.to_string())
            }
        };

        let body = Json(json!({ "error": error_message }));

        (status, body).into_response()
    }
}

impl From<tower::timeout::error::Elapsed> for AppError {
    fn from(inner: tower::timeout::error::Elapsed) -> Self {
        AppError::Elapsed(inner)
    }
}

impl From<tokio::time::error::Elapsed> for AppError {
    fn from(inner: tokio::time::error::Elapsed) -> Self {
        AppError::Tokio(TokioError::Elapsed(inner))
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(inner: tokio::task::JoinError) -> Self {
        AppError::Tokio(TokioError::JoinError(inner))
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(inner: mongodb::error::Error) -> Self {
        AppError::Mongo(inner)
    }
}

async fn connect_mongo() -> Result<Client, AppError> {
    let options = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017",
        ResolverConfig::cloudflare(),
    )
    .await?;

    let client = Client::with_options(options)?;
    println!("weha");
    // let db = client.database("rusty");

    // Ok(db)
    Ok(client)
}
