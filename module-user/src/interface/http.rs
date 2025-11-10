use std::sync::Arc;

use axum::{extract::State, routing::get};

use crate::{
    command::SignupCommand, interface::UserServiceProvider,
    service::UserService,
};

async fn hello<T>(State(container): State<Arc<T>>) -> String
where
    T: UserServiceProvider,
{
    container.user_service().hello().await
}

async fn signup<T>(State(container): State<Arc<T>>) -> String
where
    T: UserServiceProvider,
{
    let _ = container
        .user_service()
        .signup(&SignupCommand::Local {
            username: "".into(),
            email: "".into(),
            password: "".into(),
        })
        .await;
    "test signup".to_string()
}

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: UserServiceProvider + 'static,
{
    axum::Router::new()
        .route("/hello", get(hello::<T>))
        .route("/signup", get(signup::<T>))
        .with_state(t)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::command::{LoginCommand, SignupCommand};
    use crate::entity;
    use crate::service::UserService;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    pub struct TestUserService {}

    #[async_trait::async_trait]
    impl UserService for TestUserService {
        async fn hello(&self) -> String {
            "test hello".into()
        }
        async fn signup(
            &self,
            _command: &SignupCommand,
        ) -> stardust_common::Result<entity::UserAggregate> {
            unimplemented!()
        }
        async fn login(
            &self,
            _command: &LoginCommand,
        ) -> stardust_common::Result<entity::UserAggregate> {
            unimplemented!()
        }
    }

    pub struct Container<US: UserService> {
        user_service: Arc<US>,
    }

    impl<US: UserService> Container<US> {
        pub fn new(user_service: Arc<US>) -> Self {
            Self { user_service }
        }
    }

    impl<US: UserService> super::UserServiceProvider for Container<US> {
        type UserService = US;

        fn user_service(&self) -> Arc<Self::UserService> {
            self.user_service.clone()
        }
    }

    #[tokio::test]
    async fn test_hello() {
        let service = Arc::new(TestUserService {});
        let container = Arc::new(Container::new(service));
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
        let bodybytes =
            axum::body::to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let bodystring = String::from_utf8(bodybytes.to_vec()).unwrap();
        println!("{:?}", bodystring);
    }
}
