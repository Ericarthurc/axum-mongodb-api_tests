use std::sync::Arc;

use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use mongodb::Database;
use tower::Service;

use crate::State;

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

        let extensions = req.extensions_mut().get::<Arc<State>>();

        let extensions = extensions.unwrap();
        println!("Middleware: {}", extensions.name);

        Box::pin(async move {
            let res: Response = inner.call(req).await?;

            println!("`MyMiddleware` received the response");

            Ok(res)
        })
    }
}
