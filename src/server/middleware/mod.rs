pub mod make_cors_middleware;

use anyhow::Error;
use futures::Future;
use hyper::{Body, Request, Response};
use std::convert::TryFrom;
use std::pin::Pin;

use crate::config::Config;

use self::make_cors_middleware::make_cors_middleware;

pub type MiddlewareBefore = Box<dyn Fn(&mut Request<Body>) + Send + Sync>;
pub type MiddlewareAfter = Box<dyn Fn(&mut Response<Body>) + Send + Sync>;
pub type Handler = Box<
    dyn Fn(Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + Sync>>
        + Send
        + Sync,
>;

pub struct Middleware {
    before: Vec<MiddlewareBefore>,
    after: Vec<MiddlewareAfter>,
}

impl Middleware {
    /// Appends a middleware function to run before handling the
    /// HTTP Request
    #[allow(dead_code)]
    pub fn before(&mut self, middleware: MiddlewareBefore) {
        self.before.push(middleware);
    }

    /// Appends a middleware function to run after handling the
    /// HTTP Request. Thus, functions appended after will receive
    /// the handler's HTTP Response instead
    pub fn after(&mut self, middleware: MiddlewareAfter) {
        self.after.push(middleware);
    }

    /// Runs functions from the chain that must run before
    /// executing the handler (applied to the HTTP Request).
    /// Then performs the handler operations on the HTTP Request
    /// and finally executes the functions on the "after" chain
    /// with the HTTP Response from the handler
    pub async fn handle(&self, mut request: Request<Body>, handler: Handler) -> Response<Body> {
        for fx in self.before.iter() {
            fx(&mut request);
        }

        let mut response = handler(request).await;

        for fx in self.after.iter() {
            fx(&mut response);
        }

        response
    }
}

impl Default for Middleware {
    fn default() -> Self {
        Middleware {
            before: Vec::new(),
            after: Vec::new(),
        }
    }
}

impl TryFrom<Config> for Middleware {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let mut middleware = Middleware::default();

        if config.cors().is_some() {
            let func = make_cors_middleware(config);

            middleware.after(func);
        }

        Ok(middleware)
    }
}

// mod tests {
//     use std::str::FromStr;

//     use super::*;

//     fn fake_handler(_: Request<Body>) -> Pin<Box<dyn Future<Output = Response<Body>>>> {
//         let response = Response::builder()
//             .status(hyper::StatusCode::OK)
//             .body(Body::empty())
//             .unwrap();

//         Box::pin(futures::future::ready(response))
//     }

//     #[tokio::test]
//     async fn runs_chain_after() {
//         // Create a new `ResponseTransformer` we can use to
//         // transform our `Response`
//         let mut middleware_chain = Middleware::default();

//         // Create a `Response` for demo purposes
//         let request = Request::builder().body(Body::empty()).unwrap();

//         // Append the `with_cors` transformer to the `ResponseTransformer`
//         middleware_chain.after(Box::new(with_cors_allow_all::with_cors_allow_all));

//         // Process the response to have desired transformations
//         let response = middleware_chain
//             .handle(request, Box::new(fake_handler))
//             .await;
//         let response_headers = response.headers();

//         assert_eq!(
//             response_headers
//                 .get(hyper::header::ACCESS_CONTROL_ALLOW_ORIGIN)
//                 .unwrap()
//                 .to_str()
//                 .unwrap(),
//             "*"
//         );
//         assert_eq!(
//             response_headers
//                 .get(hyper::header::ACCESS_CONTROL_ALLOW_METHODS)
//                 .unwrap()
//                 .to_str()
//                 .unwrap(),
//             "GET, POST, PUT, PATCH, DELETE"
//         );
//         assert_eq!(
//             response_headers
//                 .get(hyper::header::ACCESS_CONTROL_ALLOW_HEADERS)
//                 .unwrap()
//                 .to_str()
//                 .unwrap(),
//             "Content-Type, Content-Length, Origin"
//         );
//     }
// }
