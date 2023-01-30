use http::{Request, Response};
use hyper::Body;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::addon::logger::Logger;

use super::MiddlewareAfter;

pub fn make_logger_middleware() -> MiddlewareAfter {
    let logger = Arc::new(Mutex::new(Logger::new()));

    Box::new(
        move |request: Arc<Mutex<Request<Body>>>, response: Arc<Mutex<Response<Body>>>| {
            let logger = Arc::clone(&logger);

            Box::pin(async move {
                let mut logger = logger.lock().await;

                if let Err(error) = logger.log(request, response).await {
                    eprintln!("{error:#?}");
                }

                Ok(())
            })
        },
    )
}
