use std::sync::Arc;

use axum::extract::State;

use crate::{query, service::SampleService};

async fn hello<C>(service_container: State<Arc<C>>) -> String
where
    C: crate::Container,
{
    let sample = service_container
        .sample_service()
        .find_sample(&query::FindSampleQuery {
            name: "nametest".into(),
        })
        .await;
    let sample = sample.unwrap();
    sample.name
}

pub fn routes<C>(service_container: Arc<C>) -> axum::Router
where
    C: crate::Container + 'static,
{
    axum::Router::new()
        .route("/hello", axum::routing::get(hello::<C>))
        .with_state(service_container)
}
