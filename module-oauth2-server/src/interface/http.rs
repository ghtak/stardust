use std::sync::Arc;

use crate::interface::ServiceProvider;

pub fn routes<T>(t: Arc<T>) -> axum::Router
where
    T: ServiceProvider + 'static,
{
    axum::Router::new().with_state(t)
}
