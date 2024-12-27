use std::sync::Arc;

use anyhow::{Context, Ok};

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tokio::net;

use crate::{config::Settings, domain::blog::ports::BlogService};

use super::{
    handlers::{
        batch_delete_post, create_post, create_user, delete_post, get_user, list_post, login,
        update_post,
    },
    middlewares::auth,
};

#[derive(Debug, Clone)]
pub struct AppState<BS: BlogService> {
    pub blog_service: Arc<BS>,
    pub config: Settings,
}

pub struct HttpServer {
    router: Router,
    listener: net::TcpListener,
}

impl HttpServer {
    pub async fn new(blog_service: impl BlogService, config: Settings) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );
        let state = AppState {
            blog_service: Arc::new(blog_service),
            config: config.clone(),
        };
        let router = Router::new()
            .nest("/api", api_routes(state.clone()))
            .layer(trace_layer)
            .with_state(state);
        let application_settings = config.application.clone();

        let listener = net::TcpListener::bind(format!(
            "{}:{}",
            application_settings.host, application_settings.port
        ))
        .await
        .with_context(|| format!("failed to listen on {}", application_settings.port))?;

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

fn api_routes<BS>(state: AppState<BS>) -> Router<AppState<BS>>
where
    BS: BlogService + 'static,
{
    Router::new()
        .nest(
            "/auth",
            Router::new().route("/login", post(login::login::<BS>)),
        )
        .nest(
            "/users",
            Router::new()
                .route("/", post(create_user::create_user::<BS>))
                .route("/:username", get(get_user::get_user::<BS>))
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth_middleware::<BS>,
                )),
        )
        .nest(
            "/posts",
            Router::new()
                .route("/", post(create_post::create_post::<BS>))
                .route("/", get(list_post::list_post::<BS>))
                .route("/:id", put(update_post::update_post::<BS>))
                .route("/:id", delete(delete_post::delete_post::<BS>))
                .route("/", delete(batch_delete_post::batch_delete_post::<BS>))
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth::auth_middleware::<BS>,
                )),
        )
}
