use std::sync::Arc;

use anyhow::{Context, Ok};

use axum::{routing::post, Router};
use tokio::net;

use crate::{config::ApplicationSettings, domain::blog::ports::BlogService};

use super::handlers::create_post;

#[derive(Debug, Clone)]
pub struct AppState<BS: BlogService> {
    pub blog_service: Arc<BS>,
}

pub struct HttpServer {
    router: Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new(
        blog_service: impl BlogService,
        config: ApplicationSettings,
    ) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );
        let state = AppState {
            blog_service: Arc::new(blog_service),
        };
        let router = Router::new()
            .nest("/api", api_routes())
            .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("{}:{}", config.host, config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes<BS: BlogService>() -> Router<AppState<BS>> {
    Router::new().route("/posts", post(create_post::create_post::<BS>))
}
