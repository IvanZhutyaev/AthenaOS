use crate::handlers::ApiHandlers;
use crate::routes::create_router;
use athena_core::system::AthenaSystem;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct ApiServer {
    system: Arc<AthenaSystem>,
}

impl ApiServer {
    pub fn new(system: Arc<AthenaSystem>) -> Self {
        Self { system }
    }

    pub async fn start(&self, addr: SocketAddr) -> anyhow::Result<()> {
        let handlers = Arc::new(ApiHandlers {
            system: self.system.clone(),
        });

        let app = create_router(handlers).layer(CorsLayer::permissive());

        tracing::info!("Starting API server on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

