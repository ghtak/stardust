use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse};

pub struct Container<US> {
    pub config: stardust_common::config::Config,
    pub database: stardust_db::Database,
    user_service: Arc<US>,
}

impl<US> Container<US>
where
    US: module_user::service::UserService,
{
    pub fn new(
        config: stardust_common::config::Config,
        database: stardust_db::Database,
        user_service: Arc<US>,
    ) -> Self {
        Self {
            config,
            database,
            user_service,
        }
    }
}

impl<US> module_user::interface::UserServiceProvider for Container<US>
where
    US: module_user::service::UserService,
{
    type UserService = US;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_service.clone()
    }
}

impl<US> stardust_interface::http::CommonErrorToResponse for Container<US> {
    fn into_response(
        error: stardust_common::Error,
    ) -> axum::response::Response {
        let (statuscode, message) = match error {
            stardust_common::Error::AlreadyExists => {
                (StatusCode::CONFLICT, error.to_string())
            }
            stardust_common::Error::Unauthorized => {
                (StatusCode::UNAUTHORIZED, error.to_string())
            }
            stardust_common::Error::Forbidden => {
                (StatusCode::FORBIDDEN, error.to_string())
            }
            stardust_common::Error::NotFound => {
                (StatusCode::NOT_FOUND, error.to_string())
            }
            stardust_common::Error::InvalidParameter(e) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        };
        let body = serde_json::json!({
            "code": statuscode.as_u16(),
            "message": message
        });
        (statuscode, axum::Json(body)).into_response()
    }
}
