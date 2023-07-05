use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct MonitoringLayer;

impl<S> Layer<S> for MonitoringLayer {
    type Service = MonitoringMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        MonitoringMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct MonitoringMiddleware<S> {
    inner: S,
}
struct RequestInfo {
    method: String,
    path: String,
}

impl<S> Service<Request<Body>> for MonitoringMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

 fn call(&mut self, request: Request<Body>) -> Self::Future {
        let start_time = Instant::now();
        let request_info = RequestInfo {
            method: request.method().to_string(),
            path: request.uri().path().to_string(),
        };

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            let elapsed = start_time.elapsed();

            // Access request information from request_info
            println!(
                "Request: {} {} Response: {} Elapsed: {:?}",
                request_info.method,
                request_info.path,
                response.status(),
                elapsed,
            );

            Ok(response)
        })
    }
}
