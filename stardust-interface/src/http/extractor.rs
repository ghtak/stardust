use std::{marker::PhantomData, ops::Deref};

use axum::{
    extract::{
        FromRequest, FromRequestParts, Request,
        rejection::{JsonRejection, PathRejection},
    },
    http::request::Parts,
    response::IntoResponse,
};
use serde::de::DeserializeOwned;

pub struct Json<T, E>(pub T, pub PhantomData<E>);

impl<S, T, E> FromRequest<S> for Json<T, E>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
    E: IntoResponse + From<JsonRejection>,
{
    type Rejection = E;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0, PhantomData)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

impl<T, E> Deref for Json<T, E> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Path<T, E>(pub T, pub PhantomData<E>);

impl<S, T, E> FromRequestParts<S> for Path<T, E>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
    E: IntoResponse + From<PathRejection>,
{
    type Rejection = E;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0, PhantomData)),
            Err(rejection) => Err(rejection.into()),
        }
    }
}

impl<T, E> Deref for Path<T, E> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        Router,
        body::Body,
        http::{Request, StatusCode},
        response::Response,
        routing::{get, post},
    };
    use serde::{Deserialize, Serialize};
    use tower::ServiceExt;

    use super::*;

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct TestPayload {
        name: String,
        value: i32,
    }

    #[derive(Debug)]
    enum TestApiError {
        Json(JsonRejection),
        Path(PathRejection),
    }

    impl From<JsonRejection> for TestApiError {
        fn from(rejection: JsonRejection) -> Self {
            Self::Json(rejection)
        }
    }

    impl From<PathRejection> for TestApiError {
        fn from(rejection: PathRejection) -> Self {
            Self::Path(rejection)
        }
    }

    impl IntoResponse for TestApiError {
        fn into_response(self) -> Response {
            match self {
                TestApiError::Json(rejection) => rejection.into_response(),
                TestApiError::Path(rejection) => rejection.into_response(),
            }
        }
    }

    #[tokio::test]
    async fn test_json() {
        async fn handler(Json(payload, _): Json<TestPayload, TestApiError>) -> impl IntoResponse {
            axum::Json(payload)
        }

        let app = Router::new().route("/", post(handler));

        // Success case
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name": "test", "value": 123}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Failure case (invalid JSON)
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name": "test", "value": "abc"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_path() {
        async fn handler(Path(id, _): Path<i32, TestApiError>) -> String {
            id.to_string()
        }

        let app = Router::new().route("/{id}", get(handler));

        // Success case
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/123").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(&body[..], b"123");

        // Failure case
        let response =
            app.oneshot(Request::builder().uri("/abc").body(Body::empty()).unwrap()).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
