use axum::{routing::get, AddExtensionLayer, Router};
use database::DB;
use dotenv::dotenv;
use errors::AppError;

use std::env;
use std::net::SocketAddr;

use std::sync::Arc;
use tower::ServiceBuilder;

mod database;
mod errors;
mod handlers;
mod middleware;
mod models;

#[derive(Debug)]
pub struct State {
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
        .route("/", get(handlers::api::handler))
        .layer(axum_extra::middleware::from_fn(middleware::util::test));

    let app = Router::new()
        .fallback(get(handlers::handler_404))
        .nest("/api", api_routes)
        .route("/taco", get(handlers::handler_taco))
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
