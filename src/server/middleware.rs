use std::pin::Pin;

use futures::Future;

use super::{HttpRequest, HttpResponse};

pub type MiddlewareBefore = Box<
    dyn Fn(HttpRequest) -> Pin<Box<dyn Future<Output = Result<HttpRequest, HttpResponse>> + Send + Sync>>
        + Send
        + Sync,
>;

pub type MiddlewareAfter = Box<
    dyn Fn(
            HttpResponse,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, HttpResponse>> + Send + Sync>>
        + Send
        + Sync,
>;

#[derive(Default)]
pub struct Middleware {
    before: Vec<MiddlewareBefore>,
    after: Vec<MiddlewareAfter>,
}

impl Middleware {
    pub fn new() -> Self {
        Middleware::default()
    }

    pub fn before(&mut self, middleware: MiddlewareBefore) {
        self.before.push(middleware);
    }

    pub fn after(&mut self, middleware: MiddlewareAfter) {
        self.after.push(middleware);
    }

    pub async fn handle_before(&self, request: HttpRequest) -> Result<HttpRequest, HttpResponse> {
        let mut next = request;

        for fx in self.before.iter() {
            match fx(next).await {
                Ok(next_req) => next = next_req,
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(next)
    }

    pub async fn handle_after(&self, response: HttpResponse) -> Result<HttpResponse, HttpResponse> {
        let mut next = response;

        for fx in self.after.iter() {
            match fx(next).await {
                Ok(next_res) => next = next_res,
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(next)
    }
}
