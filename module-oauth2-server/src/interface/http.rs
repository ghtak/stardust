use std::sync::Arc;

use axum::{
    extract::State,
    routing::{delete, get, post},
};
use module_user::interface::user::AdminUser;

use crate::interface::{ServiceProvider, dto};

async fn create_client<T>(
    State(_): State<Arc<T>>,
    AdminUser(_): AdminUser,
    axum::Json(_): axum::Json<dto::CreateOAuth2ClientRequest>,
) -> String
where
    T: ServiceProvider,
{
    unimplemented!()
}

async fn get_clients<T>(State(_): State<Arc<T>>) -> String
//-> Result<ApiResponse<Vec<dto::ClientDto>>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    unimplemented!()
}

async fn delete_client<T>(State(_): State<Arc<T>>) -> String
where
    T: ServiceProvider,
{
    unimplemented!()
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
    T: ServiceProvider + module_user::interface::ServiceProvider + 'static,
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
