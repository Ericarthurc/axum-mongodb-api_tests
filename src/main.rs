use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, AddExtensionLayer,
    Router,
};
use database::DB;
use dotenv::dotenv;
use errors::AppError;
use mongodb::{bson::doc, Database};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;

mod database;
mod errors;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = DB::new(
        &env::var("MONGO_URI").unwrap(),
        &env::var("MONGO_DATABASE").unwrap(),
    )
    .await?;

    let api_routes = Router::new().route("/", get(handler));

    let app = Router::new()
        .fallback(get(handler_404))
        .nest("/api", api_routes)
        .layer(AddExtensionLayer::new(db.mongo_db));

    let addr = SocketAddr::from((
        [127, 0, 0, 1],
        env::var("PORT").unwrap().parse::<u16>().unwrap(),
    ));
    println!("Sever: {}", addr);
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}

async fn handler(Extension(mongo_db): Extension<Database>) -> Result<impl IntoResponse, AppError> {
    let collection = mongo_db.collection::<Book>("ok");

    let handle = tokio::task::spawn(async move {
        for collection_name in mongo_db.list_collection_names(None).await {
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
