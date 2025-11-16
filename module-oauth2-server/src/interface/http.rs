use std::sync::Arc;

use axum::{
    extract::State,
    routing::{delete, get, post},
};
use module_user::interface::user::AdminUser;
use stardust_interface::http::ApiResponse;

use crate::{
    command,
    interface::{ServiceProvider, dto},
    query,
    service::OAuth2ClientService,
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

async fn oauth2_authorize<T>(State(_): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    unimplemented!()
}

async fn oauth2_login<T>(State(_): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    unimplemented!()
}

async fn oauth2_token<T>(State(_): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    unimplemented!()
}

async fn oauth2_me<T>(State(_): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    unimplemented!()
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
        .route("/oauth2/login", get(oauth2_login::<T>))
        .route("/oauth2/token", post(oauth2_token::<T>))
        .route("/oauth2/me", get(oauth2_me::<T>))
        .with_state(t)
}
