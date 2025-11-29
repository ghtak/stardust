use std::sync::Arc;

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{StatusCode, header},
};
use stardust_interface::http::ApiResponse;

use crate::{entity, interface::container::ServiceContainer, query, service::OAuth2AuthorizationService};

#[derive(Debug)]
pub struct OAuth2User(pub entity::OAuthUserAggregate);

impl<S> FromRequestParts<Arc<S>> for OAuth2User
where
    S: ServiceContainer + Send + Sync,
    S::OAuth2AuthorizationService: OAuth2AuthorizationService,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        let unauthorized = ApiResponse::error(axum::http::StatusCode::UNAUTHORIZED, "Unauthorized");
        let Some(authorization) =
            parts.headers.get(header::AUTHORIZATION).and_then(|h| h.to_str().ok())
        else {
            return Err(unauthorized);
        };

        if !authorization.starts_with("Bearer ") {
            return Err(unauthorized);
        }

        let Some(access_token) = authorization.strip_prefix("Bearer ") else {
            return Err(unauthorized);
        };
        let entity = state
            .oauth2_authorization_service()
            .find_user(&query::FindOAuth2UserQuery { access_token })
            .await?;

        let Some(entity) = entity else {
            return Err(unauthorized);
        };

        Ok(Self(entity))
    }
}

impl<S> OptionalFromRequestParts<Arc<S>> for OAuth2User
where
    S: ServiceContainer + Send + Sync,
    S::OAuth2AuthorizationService: OAuth2AuthorizationService,
{
    type Rejection = ApiResponse<()>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Option<Self>, Self::Rejection> {
        match <OAuth2User as FromRequestParts<Arc<S>>>::from_request_parts(parts, state).await {
            Ok(user) => Ok(Some(user)),
            Err(e) if e.code == StatusCode::UNAUTHORIZED => Ok(None),
            Err(e) => Err(e),
        }
    }
}
