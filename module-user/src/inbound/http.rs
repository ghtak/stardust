use std::sync::Arc;

use axum::{extract::State, routing::get};

pub trait UserContainer: Sync + Send {
    fn user_service(&self) -> Arc<impl UserService>;
}

use crate::service::UserService;

async fn hello<T>(State(container): State<Arc<T>>) -> String
where
    T: UserContainer,
{
    container.user_service().hello().await
}

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: UserContainer + 'static,
{
    axum::Router::new().route("/hello", get(hello::<T>)).with_state(t)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::internal::UserServiceImpl;
    use crate::service::UserService;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    pub struct UserContainerImpl<US: UserService> {
        user_service: Arc<US>,
    }

    impl<US: UserService> UserContainerImpl<US> {
        pub fn new(user_service: Arc<US>) -> Self {
            Self { user_service }
        }
    }

    impl<US: UserService> super::UserContainer for UserContainerImpl<US> {
        fn user_service(&self) -> Arc<impl UserService> {
            self.user_service.clone()
        }
    }

    #[tokio::test]
    async fn test_hello() {
        let config = stardust_common::config::Config::test_config();
        let database =
            stardust_db::Database::open(&config.database).await.unwrap();
        let service = Arc::new(UserServiceImpl::new(database));
        let container = Arc::new(UserContainerImpl::new(service));

        let router = super::routes(container.clone());
        let resp = router
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        println!("{:?}", resp);
    }
}
