use std::{sync::Arc, time::Duration};

use axum::{http::Request, response::IntoResponse};
use axum_extra::middleware::Next;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RouteStat {
    route: String,
    hits: u32,
}

use crate::errors::AppError;
use crate::State;

pub async fn test<B>(req: Request<B>, next: Next<B>) -> Result<impl IntoResponse, AppError> {
    // let extensions = req.extensions().get::<Arc<State>>().unwrap();
    // let state = Arc::clone(extensions);
    // let collection = state.mongo.mongo_db.collection::<RouteStat>("routestat");

    // let route_stat = RouteStat {
    //     route: req.uri().to_string(),
    //     hits: 1,
    // };

    // println!("test middlware");

    // let handle = tokio::task::spawn(async move {
    //     tokio::time::sleep(Duration::from_secs(5)).await;
    //     collection.insert_one(route_stat, None).await
    // });

    // tokio::time::timeout(Duration::from_secs(5), handle).await???;

    Ok(next.run(req).await)
}
