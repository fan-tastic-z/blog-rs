use crate::{
    domain::blog::{models::users::User, ports::BlogService},
    inbound::http::{http_server::AppState, response::ApiError},
};
use axum::{
    extract::{OriginalUri, Request, State},
    middleware::Next,
    response::Response,
};
use tracing::info;

pub async fn permission_middleware<BS: BlogService>(
    State(state): State<AppState<BS>>,
    OriginalUri(original_uri): OriginalUri,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let user = request
        .extensions()
        .get::<User>()
        .ok_or_else(|| ApiError::PermissionDenied("permission denied".to_string()))?;
    let sub = user.username.clone();
    let obj = original_uri.path().to_string();
    let act = request.method().to_string();
    info!("permission check: {sub} {obj} {act}");
    let permission = state
        .blog_service
        .check_permission(&sub, &obj, &act)
        .await
        .map_err(ApiError::from)?;
    if !permission {
        return Err(ApiError::PermissionDenied("permission denied".to_string()));
    }
    Ok(next.run(request).await)
}
