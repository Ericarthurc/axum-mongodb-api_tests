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
mod middleware;

#[derive(Debug)]
struct State {
    db: DB,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = DB::new(
        &env::var("MONGO_URI").unwrap(),
        &env::var("MONGO_DATABASE").unwrap(),
    )
    .await?;

    let shared_state = Arc::new(Mutex::new(State {
        db,
        name: "Eric".to_string(),
    }));

    let api_routes = Router::new().route("/", get(handler));

    let app = Router::new()
        .fallback(get(handler_404))
        .nest("/api", api_routes)
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(shared_state))
                .layer(layer_fn(|inner| MyMiddleware { inner })),
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

async fn handler(
    Extension(state): Extension<Arc<Mutex<State>>>,
) -> Result<impl IntoResponse, AppError> {
    let state = Arc::clone(&state);
    let mut state = state.lock().await;
    let collection = state.db.mongo_db.collection::<Book>("ok");

    println!("{}", state.name);

    state.name = "Timmy!".to_string();

    let handle = tokio::task::spawn(async move {
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

    Ok((StatusCode::FOUND, state.name.to_owned()))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
