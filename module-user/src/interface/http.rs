use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    routing::{get, post},
};
use stardust_interface::http::{ApiResponse, session};
use tower_sessions::Session;

use crate::{
    command::{self, CreateApiKeyCommand},
    entity,
    interface::{container::ServiceContainer, dto, extract::AuthUser},
    query,
    service::{ApiKeyService, UserService},
};

async fn signup<T>(
    State(container): State<Arc<T>>,
    Json(signup_request): Json<dto::SignupRequest>,
) -> Result<ApiResponse<dto::UserDto>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    let command = signup_request.into();
    let user_aggregate: entity::UserAggregate =
        container.user_service().signup(&command).await.map_err(|e| ApiResponse::from(e))?;
    Ok(ApiResponse::with(dto::UserDto {
        id: user_aggregate.user.id,
        username: user_aggregate.user.username,
        email: user_aggregate.user.email,
        role: user_aggregate.user.role.to_string(),
        status: user_aggregate.user.status.to_string(),
    }))
}

async fn login<T>(
    State(container): State<Arc<T>>,
    session: Session,
    Json(request): Json<dto::LoginRequest>,
) -> Result<ApiResponse<dto::UserDto>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    let command = request.into();
    let user_aggregate: entity::UserAggregate =
        container.user_service().login(&command).await.map_err(|e| ApiResponse::from(e))?;

    session::store_user(&session, &user_aggregate.user).await?;
    Ok(ApiResponse::with(dto::UserDto {
        id: user_aggregate.user.id,
        username: user_aggregate.user.username,
        email: user_aggregate.user.email,
        role: user_aggregate.user.role.to_string(),
        status: user_aggregate.user.status.to_string(),
    }))
}

async fn logout<T>(State(_): State<Arc<T>>, s: Session) -> Result<ApiResponse<()>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    session::remove_user(&s).await?;
    Ok(ApiResponse::ok())
}

async fn me<T>(
    State(_): State<Arc<T>>,
    AuthUser(authuser): AuthUser,
) -> Result<ApiResponse<dto::UserDto>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    Ok(ApiResponse::with(dto::UserDto {
        id: authuser.id,
        username: authuser.username,
        email: authuser.email,
        role: authuser.role.to_string(),
        status: authuser.status.to_string(),
    }))
}

async fn create_apikey<T>(
    State(container): State<Arc<T>>,
    AuthUser(user): AuthUser,
    axum::Json(req): axum::Json<dto::CreateApiKeyRequest>,
) -> Result<ApiResponse<dto::CreateApiKeyResponse>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    let command = CreateApiKeyCommand {
        user_id: user.id,
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
    State(container): State<Arc<T>>,
    AuthUser(user): AuthUser,
) -> Result<ApiResponse<Vec<dto::ApiKeyDto>>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    let result = container
        .apikey_service()
        .find_apikeys(&query::FindApiKeysQuery { user_id: user.id })
        .await?;
    Ok(ApiResponse::with(
        result.into_iter().map(|a| a.into()).collect(),
    ))
}

async fn deactivate_apikey<T>(
    State(container): State<Arc<T>>,
    AuthUser(user): AuthUser,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<ApiResponse<dto::ApiKeyDto>, ApiResponse<()>>
where
    T: ServiceContainer,
{
    let result = container
        .apikey_service()
        .deactivate_apikey(&command::DeactivateApiKeyCommand {
            apikey_id: id,
            request_user_id: user.id,
        })
        .await?;
    Ok(ApiResponse::with(result.into()))
}

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: ServiceContainer + 'static,
{
    axum::Router::new()
        .route("/auth/user/signup", post(signup::<T>))
        .route("/auth/user/login", post(login::<T>))
        .route("/auth/user/logout", post(logout::<T>))
        .route("/auth/user/me", get(me::<T>))
        .route(
            "/auth/user/apikey",
            get(get_apikey::<T>).post(create_apikey::<T>),
        )
        .route(
            "/auth/user/apikey/{id}",
            get(get_apikey::<T>).delete(deactivate_apikey),
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
