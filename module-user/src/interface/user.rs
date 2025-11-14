use std::ops::Deref;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::StatusCode,
};
use stardust_interface::http::ApiResponse;
use tower_sessions::Session;

use crate::entity::UserAggregate;

#[derive(Debug)]
pub struct AuthUser(pub UserAggregate);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        s: &S,
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, s)
            .await
            .map_err(|e| ApiResponse::error(e.0, e.1))?;
        match stardust_interface::http::session::get_user::<UserAggregate>(&session).await? {
            Some(user) => Ok(Self(user)),
            _ => Err(ApiResponse::error(StatusCode::UNAUTHORIZED, "Unauthorized")),
        }
    }
}

impl<S> OptionalFromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <AuthUser as FromRequestParts<S>>::from_request_parts(parts, state).await {
            Ok(user) => Ok(Some(user)),
            Err(e) if e.code == StatusCode::UNAUTHORIZED => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl Deref for AuthUser {
    type Target = UserAggregate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
