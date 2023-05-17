use axum::{Router, routing::get, response::{IntoResponse, Html}};
use crate::cli::Cli;
use tracing::{debug, info, Level};
use tracing_subscriber::EnvFilter;

pub struct Server(pub Router);

impl Server {
    pub fn router(self) -> Router {
        self.0
    }
}     

impl From<Cli> for Server {
    fn from(value: Cli) -> Self {

        let filter = EnvFilter::from_default_env()
            .add_directive(Level::INFO.into())
            .add_directive(Level::DEBUG.into());

        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .init();

        let mut app = Router::new();

        if value.port == 3000 {
            app = app.route("/", get(handler_default));
        } else {
            app = app.route("/", get(handler_custom));
        }

        Self(app)
    }
}

async fn handler_default() -> impl IntoResponse{
   debug!("A request has been received on the default route");
   Html("Hello <strong>Rust!!!</strong>")
}

async fn handler_custom() -> impl IntoResponse {
    info!("A request has been received on a custom route");
    Html("Hello <strong>Rust!!!</strong>")
}