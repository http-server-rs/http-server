use axum::Router;

use crate::cli::Cli;

pub struct Server(Router);

impl Server {
    pub fn router(self) -> Router {
        self.0
    }
}

impl From<Cli> for Server {
    fn from(value: Cli) -> Self {
        let mut app = Router::new();

        if value.port == 7878 {
            app = app.route("/", axum::routing::get(|| async { "Hello, Rust!" }));
        } else {
            app = app.route("/", axum::routing::get(|| async { "Hello, World!" }));
        }

        Self(app)
    }
}
