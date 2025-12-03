use std::{marker::PhantomData, sync::Arc};

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::header,
    response::IntoResponse,
};

use crate::{entity, query, service::OAuth2AuthorizationService};

#[derive(Debug)]
pub struct OAuth2User<R>(pub entity::OAuthUserAggregate, pub PhantomData<R>);

impl<S, R> OptionalFromRequestParts<Arc<S>> for OAuth2User<R>
where
    S: crate::Container + Send + Sync,
    S::OAuth2AuthorizationService: OAuth2AuthorizationService,
    R: From<stardust::Error> + IntoResponse + Send,
{
    type Rejection = R;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Option<Self>, Self::Rejection> {
        let unauthorized = R::from(stardust::Error::Unauthorized);
        let Some(authorization) = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
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
            .await
            .map_err(R::from)?;

        let Some(entity) = entity else {
            return Err(unauthorized);
        };

        Ok(Some(Self(entity, PhantomData)))
    }
}

impl<S, R> FromRequestParts<Arc<S>> for OAuth2User<R>
where
    S: crate::Container + Send + Sync,
    S::OAuth2AuthorizationService: OAuth2AuthorizationService,
    R: From<stardust::Error> + IntoResponse + Send,
{
    type Rejection = R;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<S>,
    ) -> Result<Self, Self::Rejection> {
        match <OAuth2User<R> as OptionalFromRequestParts<Arc<S>>>::from_request_parts(parts, state).await {
            Ok(Some(user)) => Ok(user),
            _ => Err(R::from(stardust::Error::Unauthorized)),
        }
    }
}
