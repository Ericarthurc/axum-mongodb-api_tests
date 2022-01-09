use std::{sync::Arc, time::Duration};

use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use tokio::sync::Mutex;
use tower::Service;

use crate::{Book, State};
pub mod util;

#[derive(Clone)]
pub struct MyMiddleware<S> {
    pub inner: S,
}

impl<S> Service<Request<Body>> for MyMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        println!("`MyMiddleware` called!");

        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        req.extensions_mut().insert("middleware jwt");

        let extensions = req.extensions_mut().get::<Arc<State>>().unwrap();
        let state = Arc::clone(extensions);
        let collection = state.db.mongo_db.collection::<Book>("ok");

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

        Box::pin(async move {
            tokio::time::timeout(Duration::from_secs(5), handle)
                .await
                .unwrap()
                .unwrap()
                .unwrap();

            let res: Response = inner.call(req).await?;

            println!("`MyMiddleware` received the response");

            Ok(res)
        })
    }
}
