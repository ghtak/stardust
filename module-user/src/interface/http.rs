use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    routing::{get, post},
};
use stardust_interface::http::{ApiResponse, session};
use tower_sessions::Session;

use crate::{
    command::CreateApiKeyCommand,
    entity,
    interface::{ServiceProvider, dto, user::AuthUser},
    service::{ApiKeyService, UserService},
};

async fn signup<T>(
    State(container): State<Arc<T>>,
    Json(signup_request): Json<dto::SignupRequest>,
) -> Result<ApiResponse<dto::UserDto>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    let command = signup_request.into();
    let user: entity::UserAggregate =
        container.user_service().signup(&command).await.map_err(|e| ApiResponse::from(e))?;
    Ok(ApiResponse::with(dto::UserDto {
        uids: user.accounts.iter().map(|a| a.uid.clone()).collect(),
        username: user.user.username,
        email: user.user.email,
        role: user.user.role.to_string(),
        status: user.user.status.to_string(),
    }))
}

async fn login<T>(
    State(container): State<Arc<T>>,
    session: Session,
    Json(request): Json<dto::LoginRequest>,
) -> Result<ApiResponse<dto::UserDto>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    let command = request.into();
    let user: entity::UserAggregate =
        container.user_service().login(&command).await.map_err(|e| ApiResponse::from(e))?;

    session::store_user(&session, &user).await?;
    Ok(ApiResponse::with(dto::UserDto {
        uids: user.accounts.iter().map(|a| a.uid.clone()).collect(),
        username: user.user.username,
        email: user.user.email,
        role: user.user.role.to_string(),
        status: user.user.status.to_string(),
    }))
}

async fn logout<T>(State(_): State<Arc<T>>, s: Session) -> Result<ApiResponse<()>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    session::remove_user(&s).await?;
    Ok(ApiResponse::ok())
}

async fn me<T>(
    State(_): State<Arc<T>>,
    AuthUser(authuser): AuthUser,
) -> Result<ApiResponse<dto::UserDto>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    Ok(ApiResponse::with(dto::UserDto {
        uids: authuser.accounts.iter().map(|a| a.uid.clone()).collect(),
        username: authuser.user.username,
        email: authuser.user.email,
        role: authuser.user.role.to_string(),
        status: authuser.user.status.to_string(),
    }))
}

async fn create_apikey<T>(
    State(container): State<Arc<T>>,
    AuthUser(user): AuthUser,
    axum::Json(req): axum::Json<dto::CreateApiKeyRequest>,
) -> Result<ApiResponse<dto::CreateApiKeyResponse>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    let command = CreateApiKeyCommand {
        user_id: user.user.id,
        description: req.description,
    };
    let result = container.apikey_service().create_apikey(&command).await?;
    Ok(ApiResponse::with(dto::CreateApiKeyResponse {
        id: result.related.id,
        key: result.inner,
        description: result.related.description,
    }))
}

async fn get_apikey<T>(
    State(_): State<Arc<T>>,
    AuthUser(_): AuthUser,
) -> Result<ApiResponse<String>, ApiResponse<()>>
where
    T: ServiceProvider,
{
    unimplemented!()
}

async fn delete_apikey<T>(
    State(_): State<Arc<T>>,
    AuthUser(_): AuthUser,
) -> Result<ApiResponse<String>, ApiResponse<()>>
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
        .route("/auth/user/signup", post(signup::<T>))
        .route("/auth/user/login", post(login::<T>))
        .route("/auth/user/logout", post(logout::<T>))
        .route("/auth/user/me", get(me::<T>))
        .route(
            "/auth/user/apikey",
            get(get_apikey::<T>).post(create_apikey::<T>).delete(delete_apikey),
        )
        .with_state(t)
}

#[cfg(test)]
mod tests {
    // use std::sync::Arc;

    // use crate::command::{LoginCommand, SignupCommand};
    // use crate::entity;
    // use crate::service::UserService;
    // use axum::body::Body;
    // use axum::http::Request;
    // use tower::ServiceExt;

    // pub struct TestUserService {}

    // #[async_trait::async_trait]
    // impl UserService for TestUserService {
    //     async fn hello(&self) -> String {
    //         "test hello".into()
    //     }
    //     async fn signup(
    //         &self,
    //         _command: &SignupCommand,
    //     ) -> stardust_common::Result<entity::UserAggregate> {
    //         unimplemented!()
    //     }
    //     async fn login(
    //         &self,
    //         _command: &LoginCommand,
    //     ) -> stardust_common::Result<entity::UserAggregate> {
    //         unimplemented!()
    //     }
    // }

    // pub struct Container<US: UserService> {
    //     user_service: Arc<US>,
    // }

    // impl<US: UserService> Container<US> {
    //     pub fn new(user_service: Arc<US>) -> Self {
    //         Self { user_service }
    //     }
    // }

    // impl<US: UserService> super::ServiceProvider for Container<US> {
    //     type UserService = US;

    //     fn user_service(&self) -> Arc<Self::UserService> {
    //         self.user_service.clone()
    //     }
    // }

    // #[tokio::test]
    // async fn test_hello() {
    //     let service = Arc::new(TestUserService {});
    //     let container = Arc::new(Container::new(service));
    //     let router = super::routes(container.clone());
    //     let resp = router
    //         .oneshot(Request::builder().method("GET").uri("/hello").body(Body::empty()).unwrap())
    //         .await
    //         .unwrap();
    //     let bodybytes = axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
    //     let bodystring = String::from_utf8(bodybytes.to_vec()).unwrap();
    //     println!("{:?}", bodystring);
    // }
}
