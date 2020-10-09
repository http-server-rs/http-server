use crate::file_explorer::FileExplorer;
use crate::handler::file_explorer;
use tiny_http::{Request, Response, ResponseBox};

/// The main handler acts like a router for every supported method.
/// If a method is not supported then responds with `Method Not Allowed 405`
pub fn main_handler(req: Request, fexplorer: &FileExplorer) -> (Request, ResponseBox) {
    match req.method().to_string().to_lowercase().as_str() {
        "get" => file_explorer(req, fexplorer),
        _ => (
            req,
            Response::from_string("Method Not Allowed")
                .with_status_code(405)
                .boxed(),
        ),
    }
}
