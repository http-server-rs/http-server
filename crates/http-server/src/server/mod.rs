use crate::cli::Cli;
use axum::{routing::get, Router};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

pub struct Server(pub Router);

impl Server {
    pub fn router(self) -> Router {
        self.0
    }
}

impl From<Cli> for Server {
    fn from(value: Cli) -> Self {
        let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());

        tracing_subscriber::fmt().with_env_filter(filter).init();

        let mut app = Router::new();

        if value.port == 7878 {
            app = app.route("/", get(|| async { info!("Hello, Rust!") }));
        } else {
            app = app.route("/", get(|| async { info!("Hello, World!") }));
        }

        Self(app)
    }
}
