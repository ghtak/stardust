use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::{delete, get, post},
};
use module_user::interface::user::{AdminUser, AuthUser};
use stardust_interface::http::ApiResponse;

use crate::{
    command,
    interface::{ServiceProvider, dto},
    query,
    service::{OAuth2AuthorizationService, OAuth2ClientService},
};

async fn create_client<T>(
    State(container): State<Arc<T>>,
    AdminUser(_): AdminUser,
    axum::Json(req): axum::Json<dto::CreateOAuth2ClientRequest>,
) -> Result<ApiResponse<dto::OAuth2ClientDto>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    let entity = container.oauth2_client_service().create_client(&req.into()).await?;
    Ok(ApiResponse::with(entity.into()))
}

async fn get_clients<T>(
    State(ct): State<Arc<T>>,
    AdminUser(_): AdminUser,
) -> Result<ApiResponse<Vec<dto::OAuth2ClientDto>>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    let clients = ct
        .oauth2_client_service()
        .find_clients(&query::FindOAuth2ClientQuery { client_id: None })
        .await?;
    Ok(ApiResponse::with(
        clients.into_iter().map(|c| c.into()).collect(),
    ))
}

async fn delete_client<T>(
    State(ct): State<Arc<T>>,
    AdminUser(_): AdminUser,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> ApiResponse<()>
where
    T: ServiceProvider,
{
    let result =
        ct.oauth2_client_service().delete_client(&command::DeleteOAuth2ClientCommand { id }).await;
    match result {
        Ok(_) => ApiResponse::ok(),
        Err(e) => ApiResponse::from(e),
    }
}

async fn oauth2_authorize<T>(
    State(ct): State<Arc<T>>,
    Query(req): Query<dto::OAuth2AuthorizeRequest>,
    user: Option<AuthUser>,
) -> Result<impl IntoResponse, ApiResponse<()>>
where
    T: ServiceProvider,
{
    let Some(user) = user else {
        let _ = ct.oauth2_authorization_service().verify(&req.as_verify_command()).await?;
        let callback_url = format!("/oauth2/authorize?{}", req.as_params());
        let redirect_url = format!(
            "/oauth2/login.html?callback={}",
            urlencoding::encode(&callback_url)
        );
        return Ok(Redirect::to(&redirect_url).into_response());
    };

    let entity = ct
        .oauth2_authorization_service()
        .authorize(&command::OAuth2AuthorizeCommand {
            principal: &user,
            verify_command: &req.as_verify_command(),
        })
        .await?;
    let redirect_url = format!(
        "{}?code={}&state={}",
        req.redirect_uri, entity.auth_code_value, req.state
    );
    Ok(Redirect::to(&redirect_url).into_response())
}

async fn oauth2_token<T>(State(ct): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    let _ = ct.oauth2_authorization_service();
    "".into()
}

async fn oauth2_me<T>(State(ct): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    let _ = ct.oauth2_authorization_service();
    "".into()
}

async fn oauth2_testcallback<T>(
    State(_): State<Arc<T>>,
    Query(req): Query<HashMap<String, String>>,
) -> String
where
    T: ServiceProvider,
{
    let kvstring =
        req.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
    kvstring
}

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: ServiceProvider + 'static,
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
