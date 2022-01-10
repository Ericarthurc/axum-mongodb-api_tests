use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::get, AddExtensionLayer,
    Router,
};
use database::DB;
use dotenv::dotenv;
use errors::AppError;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::{env, time::Duration};
use tower::layer::layer_fn;

use std::sync::Arc;
use tower::ServiceBuilder;

use tokio::sync::Mutex;

use middleware::MyMiddleware;

mod database;
mod errors;
mod handlers;
mod middleware;
mod models;

#[derive(Debug)]
struct State {
    mongo: DB,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let mongo = DB::new(
        &env::var("MONGO_URI").unwrap(),
        &env::var("MONGO_DATABASE").unwrap(),
    )
    .await?;

    let shared_state = Arc::new(State { mongo });

    let api_routes = Router::new()
        .route("/", get(handler))
        .layer(axum_extra::middleware::from_fn(middleware::util::test));

    let app = Router::new()
        .fallback(get(handler_404))
        .nest("/api", api_routes)
        .route("/taco", get(handler_taco))
        .layer(
            ServiceBuilder::new().layer(AddExtensionLayer::new(shared_state)),
            // .layer(axum_extra::middleware::from_fn(middleware::util::test)),
            // .layer(layer_fn(|inner| MyMiddleware { inner })),
        );

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

async fn handler(Extension(state): Extension<Arc<State>>) -> Result<impl IntoResponse, AppError> {
    let state = Arc::clone(&state);
    let collection = state.mongo.mongo_db.collection::<Book>("ok");

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

    let handle = tokio::task::spawn(async move { collection.insert_many(books, None).await });

    tokio::time::timeout(Duration::from_secs(5), handle).await???;

    Ok((StatusCode::FOUND, "bobby!"))
}

async fn handler_404() -> impl IntoResponse {
    println!("404!");
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn handler_taco() -> impl IntoResponse {
    println!("TACOS!");
    (StatusCode::OK, "nothing to see here... but TACOS!")
}
