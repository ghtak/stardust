use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    routing::{get, post},
};
use stardust::http::session;
use tower_sessions::Session;

use crate::{
    command::{self, CreateApiKeyCommand},
    entity,
    interface::{dto, extract::AuthUser},
    query,
    service::{ApiKeyService, UserService},
};

async fn signup<T>(
    State(container): State<Arc<T>>,
    Json(signup_request): Json<dto::SignupRequest>,
) -> stardust::Result<axum::Json<dto::UserDto>>
where
    T: crate::Container,
{
    let command = signup_request.into();
    let user_aggregate: entity::UserAggregate =
        container.user_service().signup(&command).await?;
    Ok(axum::Json(dto::UserDto {
        id: user_aggregate.user.id,
        username: user_aggregate.user.username,
        email: user_aggregate.user.email,
        role: user_aggregate.user.role.to_string(),
        status: user_aggregate.user.status.to_string(),
    }))
}

async fn login<T>(
    State(container): State<Arc<T>>,
    session: Session,
    Json(request): Json<dto::LoginRequest>,
) -> stardust::Result<axum::Json<dto::UserDto>>
where
    T: crate::Container,
{
    let command = request.into();
    let user_aggregate: entity::UserAggregate =
        container.user_service().login(&command).await?;

    session::store_user(&session, &user_aggregate.user).await?;
    Ok(axum::Json(dto::UserDto {
        id: user_aggregate.user.id,
        username: user_aggregate.user.username,
        email: user_aggregate.user.email,
        role: user_aggregate.user.role.to_string(),
        status: user_aggregate.user.status.to_string(),
    }))
}

async fn logout<T>(State(_): State<Arc<T>>, s: Session) -> stardust::Result<()>
where
    T: crate::Container,
{
    session::remove_user(&s).await?;
    Ok(())
}

async fn me<T>(
    State(_): State<Arc<T>>,
    AuthUser(authuser, _): AuthUser<stardust::Error>,
) -> stardust::Result<axum::Json<dto::UserDto>>
where
    T: crate::Container,
{
    Ok(axum::Json(dto::UserDto {
        id: authuser.id,
        username: authuser.username,
        email: authuser.email,
        role: authuser.role.to_string(),
        status: authuser.status.to_string(),
    }))
}

async fn create_apikey<T>(
    State(container): State<Arc<T>>,
    AuthUser(user, _): AuthUser<stardust::Error>,
    axum::Json(req): axum::Json<dto::CreateApiKeyRequest>,
) -> stardust::Result<axum::Json<dto::CreateApiKeyResponse>>
where
    T: crate::Container,
{
    let command = CreateApiKeyCommand {
        user_id: user.id,
        description: req.description,
    };
    let result = container.apikey_service().create_apikey(&command).await?;
    Ok(axum::Json(dto::CreateApiKeyResponse {
        id: result.apikey.id,
        key: result.secret,
        description: result.apikey.description,
    }))
}

async fn get_apikey<T>(
    State(container): State<Arc<T>>,
    AuthUser(user, _): AuthUser<stardust::Error>,
) -> stardust::Result<axum::Json<Vec<dto::ApiKeyDto>>>
where
    T: crate::Container,
{
    let result = container
        .apikey_service()
        .find_apikeys(&query::FindApiKeysQuery { user_id: user.id })
        .await?;
    Ok(axum::Json(result.into_iter().map(|a| a.into()).collect()))
}

async fn deactivate_apikey<T>(
    State(container): State<Arc<T>>,
    AuthUser(user, _): AuthUser<stardust::Error>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> stardust::Result<axum::Json<dto::ApiKeyDto>>
where
    T: crate::Container,
{
    let result = container
        .apikey_service()
        .deactivate_apikey(&command::DeactivateApiKeyCommand {
            apikey_id: id,
            request_user_id: user.id,
        })
        .await?;
    Ok(axum::Json(result.into()))
}

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: crate::Container + 'static,
{
    axum::Router::new()
        .route("/auth/user/signup", post(signup::<T>))
        .route("/auth/user/login", post(login::<T>))
        .route("/auth/user/logout", post(logout::<T>))
        .route("/auth/user/me", get(me::<T>))
        .route(
            "/auth/user/apikey",
            get(get_apikey::<T>).post(create_apikey::<T>),
        )
        .route(
            "/auth/user/apikey/{id}",
            get(get_apikey::<T>).delete(deactivate_apikey),
        )
        .with_state(t)
}

#[cfg(test)]
mod tests {}
