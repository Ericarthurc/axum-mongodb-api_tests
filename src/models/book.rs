use futures::StreamExt;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
}

impl Book {
    pub async fn find_all(book_collection: &Collection<Book>) -> Result<Vec<Book>, AppError> {
        let mut cursor = book_collection.find(None, None).await?;

        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(doc?);
        }
        Ok(result)
    }

    pub async fn find() {}
    pub async fn create() {}
    pub async fn update() {}
    pub async fn delete() {}
}
