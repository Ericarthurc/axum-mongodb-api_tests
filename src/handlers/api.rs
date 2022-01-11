use std::{sync::Arc, time::Duration};

use axum::{extract::Extension, http::StatusCode, response::IntoResponse, response::Json};

use crate::{errors::AppError, models::book::Book, State};

pub async fn handler(
    Extension(state): Extension<Arc<State>>,
) -> Result<impl IntoResponse, AppError> {
    let state = Arc::clone(&state);
    // let book_collection = state.mongo.mongo_book_collection;

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

    let handle = tokio::task::spawn(async move {
        state
            .mongo
            .mongo_book_collection
            .insert_many(books, None)
            .await
    });

    tokio::time::timeout(Duration::from_secs(5), handle).await???;

    // let handle_two =
    //     tokio::task::spawn(async move { Book::find_all(&state.mongo.mongo_book_collection).await });

    // let le_books = tokio::time::timeout(Duration::from_secs(5), handle_two).await???;

    Ok((StatusCode::FOUND, "hi"))
}
