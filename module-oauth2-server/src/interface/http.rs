use std::{collections::HashMap, sync::Arc};

use axum::{
    Form,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::{delete, get, post},
};
use module_user::interface::extract::{AdminUser, AuthUser};

use crate::{
    command, entity,
    interface::{dto, extract},
    query,
    service::{OAuth2AuthorizationService, OAuth2ClientService},
};

async fn create_client<T>(
    State(container): State<Arc<T>>,
    AdminUser(_user, _): AdminUser<stardust::Error>,
    axum::Json(req): axum::Json<dto::CreateOAuth2ClientRequest>,
) -> stardust::Result<axum::Json<dto::OAuth2ClientDto>>
where
    T: crate::Container,
{
    let entity =
        container.oauth2_client_service().create_client(&req.into()).await?;
    Ok(axum::Json(entity.into()))
}

async fn get_clients<T>(
    State(ct): State<Arc<T>>,
    AdminUser(_, _): AdminUser<stardust::Error>,
) -> stardust::Result<axum::Json<Vec<dto::OAuth2ClientDto>>>
where
    T: crate::Container,
{
    let clients = ct
        .oauth2_client_service()
        .find_clients(&query::FindOAuth2ClientQuery { client_id: None })
        .await?;
    Ok(axum::Json(clients.into_iter().map(|c| c.into()).collect()))
}

async fn delete_client<T>(
    State(ct): State<Arc<T>>,
    AdminUser(_, _): AdminUser<stardust::Error>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> stardust::Result<()>
where
    T: crate::Container,
{
    let result = ct
        .oauth2_client_service()
        .delete_client(&command::DeleteOAuth2ClientCommand { id })
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

async fn oauth2_authorize<T>(
    State(ct): State<Arc<T>>,
    Query(req): Query<dto::OAuth2AuthorizeRequest>,
    user: Option<AuthUser<stardust::Error>>,
) -> stardust::Result<impl IntoResponse>
where
    T: crate::Container,
{
    let Some(user) = user else {
        let _ = ct
            .oauth2_authorization_service()
            .verify(&req.as_verify_command())
            .await?;
        let callback_url = format!("/oauth2/authorize?{}", req.as_params());
        let redirect_url = format!(
            "/oauth2/login.html?callback={}",
            urlencoding::encode(&callback_url)
        );
        return Ok(Redirect::to(&redirect_url).into_response());
    };

    let entity = ct
        .oauth2_authorization_service()
        .authorize(&command::AuthorizeOAuth2Command {
            principal: &user.0,
            verify_command: &req.as_verify_command(),
            config: None,
        })
        .await?;
    let redirect_url = format!(
        "{}?code={}&state={}",
        req.redirect_uri, entity.auth_code_value, req.state
    );
    Ok(Redirect::to(&redirect_url).into_response())
}

async fn oauth2_token<T>(
    State(ct): State<Arc<T>>,
    Form(req): Form<dto::OAuth2TokenRequest>,
) -> stardust::Result<axum::Json<dto::OAuth2TokenResponse>>
where
    T: crate::Container,
{
    let token =
        ct.oauth2_authorization_service().token(&req.as_command()).await?;
    Ok(axum::Json(token.into()))
}

async fn oauth2_me<T>(
    State(_): State<Arc<T>>,
    extract::OAuth2User(user, _): extract::OAuth2User<stardust::Error>,
) -> stardust::Result<axum::Json<entity::OAuthUserAggregate>>
where
    T: crate::Container,
{
    Ok(axum::Json(user))
}

async fn oauth2_testcallback<T>(
    State(_): State<Arc<T>>,
    Query(req): Query<HashMap<String, String>>,
) -> String
where
    T: crate::Container,
{
    let kvstring = req
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&");
    kvstring
}

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: crate::Container + 'static,
{
    axum::Router::new()
        .route(
            "/oauth2/client",
            post(create_client::<T>).get(get_clients::<T>),
        )
        .route("/oauth2/client/{id}", delete(delete_client::<T>))
        .route("/oauth2/authorize", get(oauth2_authorize::<T>))
        .route("/oauth2/token", post(oauth2_token::<T>))
        .route("/oauth2/me", get(oauth2_me::<T>))
        .route("/oauth2/testcallback", get(oauth2_testcallback::<T>))
        .with_state(t)
}
