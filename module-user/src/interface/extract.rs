use std::{marker::PhantomData, ops::Deref, sync::Arc};

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    response::IntoResponse,
};
use tower_sessions::Session;

use crate::{entity::UserEntity, query, service::ApiKeyService};

pub const APIKEY_HEADER_NAME: &str = "x-apikey";

#[derive(Debug)]
pub struct AuthUser<R>(pub UserEntity, pub PhantomData<R>);

impl<S, R> OptionalFromRequestParts<Arc<S>> for AuthUser<R>
where
    S: crate::Container + Send + Sync,
    S::ApiKeyService: ApiKeyService,
    R: From<stardust::Error> + IntoResponse,
{
    type Rejection = R;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Option<Self>, Self::Rejection> {
        if let Some(key) =
            parts.headers.get(APIKEY_HEADER_NAME).and_then(|h| h.to_str().ok())
        {
            if let Some(apikey_user) = state
                .apikey_service()
                .find_user(&query::FindApiKeyUserQuery { key_hash: key })
                .await
                .map_err(R::from)?
            {
                return Ok(Some(Self(apikey_user.user, PhantomData)));
            }
        }
        let session =
            Session::from_request_parts(parts, state).await.map_err(|e| {
                R::from(stardust::Error::Unhandled(anyhow::anyhow!(
                    "from session error {:?}",
                    e
                )))
            })?;
        match stardust::http::session::get_user::<UserEntity>(&session)
            .await
            .map_err(R::from)?
        {
            Some(user) => Ok(Some(Self(user, PhantomData))),
            None => Ok(None),
        }
    }
}

impl<S, R> FromRequestParts<Arc<S>> for AuthUser<R>
where
    S: crate::Container + Send + Sync,
    S::ApiKeyService: ApiKeyService,
    R: From<stardust::Error> + IntoResponse,
{
    type Rejection = R;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        match <AuthUser<R> as OptionalFromRequestParts<Arc<S>>>::from_request_parts(
            parts, state,
        )
        .await
        {
            Ok(Some(user)) => Ok(user),
            _ => Err(R::from(stardust::Error::Unauthorized)),
        }
    }
}

impl<R> Deref for AuthUser<R> {
    type Target = UserEntity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct AdminUser<R>(pub UserEntity, pub PhantomData<R>);

impl<S, R> FromRequestParts<Arc<S>> for AdminUser<R>
where
    S: crate::Container + Send + Sync,
    S::ApiKeyService: ApiKeyService,
    R: From<stardust::Error> + IntoResponse,
{
    type Rejection = R;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        match <AuthUser<R> as FromRequestParts<Arc<S>>>::from_request_parts(
            parts, state,
        )
        .await
        {
            Ok(authuser) => {
                if authuser.0.role == crate::entity::Role::Admin {
                    Ok(Self(authuser.0, PhantomData))
                } else {
                    Err(R::from(stardust::Error::Forbidden))
                }
            }
            Err(e) => Err(e),
        }
    }
}
