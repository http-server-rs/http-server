use crate::{cli::Cli, middleware::monitoring::MonitoringLayer};
use axum::Router;
use file_explorer::FileExplorer;
use tracing::Level;
use tracing_subscriber::EnvFilter;

pub struct Server(Router);
impl Server {
    pub fn router(self) -> Router {
        self.0
    }
}

impl From<Cli> for Server {
    fn from(opts: Cli) -> Self {
        let filter = EnvFilter::from_default_env().add_directive(Level::INFO.into());

        tracing_subscriber::fmt().with_env_filter(filter).init();

        let file_explorer = FileExplorer::new(opts.root_dir);
        let app = Router::new()
            .layer(MonitoringLayer)
            .nest_service("/", file_explorer);

        Self(app)
    }
}
