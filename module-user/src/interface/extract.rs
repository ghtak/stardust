use std::{ops::Deref, sync::Arc};

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::StatusCode,
};
use stardust_interface::http::ApiResponse;
use tower_sessions::Session;

use crate::{entity::UserEntity, interface::ServiceProvider, query, service::ApiKeyService};

pub const APIKEY_HEADER_NAME: &str = "X-ApiKey";

#[derive(Debug)]
pub struct AuthUser(pub UserEntity);

impl<S> FromRequestParts<Arc<S>> for AuthUser
where
    S: ServiceProvider + Send + Sync,
    S::ApiKeyService: ApiKeyService,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        s: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        if let Some(key_hash) = parts.headers.get(APIKEY_HEADER_NAME).and_then(|h| h.to_str().ok())
        {
            if let Some(apikey_user) =
                s.apikey_service().find_user(&query::FindApiKeyUserQuery { key_hash }).await?
            {
                return Ok(Self(apikey_user.user));
            }
        }
        let session = Session::from_request_parts(parts, s)
            .await
            .map_err(|e| ApiResponse::error(e.0, e.1))?;
        match stardust_interface::http::session::get_user::<UserEntity>(&session).await? {
            Some(user) => Ok(Self(user)),
            _ => Err(ApiResponse::error(StatusCode::UNAUTHORIZED, "Unauthorized")),
        }
    }
}

impl<S> OptionalFromRequestParts<Arc<S>> for AuthUser
where
    S: ServiceProvider + Send + Sync,
    S::ApiKeyService: ApiKeyService,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <AuthUser as FromRequestParts<Arc<S>>>::from_request_parts(parts, state).await {
            Ok(user) => Ok(Some(user)),
            Err(e) if e.code == StatusCode::UNAUTHORIZED => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Deref for AuthUser {
    type Target = UserEntity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct AdminUser(pub UserEntity);

impl<S> FromRequestParts<Arc<S>> for AdminUser
where
    S: ServiceProvider + Send + Sync,
    S::ApiKeyService: ApiKeyService,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        s: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        match <AuthUser as FromRequestParts<Arc<S>>>::from_request_parts(parts, s).await {
            Ok(authuser) => {
                if authuser.0.role == crate::entity::Role::Admin {
                    Ok(Self(authuser.0))
                } else {
                    Err(ApiResponse::error(StatusCode::FORBIDDEN, "Forbidden"))
                }
            }
            Err(e) => Err(e),
        }
    }
}
