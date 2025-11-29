use std::sync::Arc;

use axum::extract::State;

use crate::{interface::container::ServiceContainer, query, service::SampleService};

async fn hello<ServiceCt>(service_container: State<Arc<ServiceCt>>) -> String
where
    ServiceCt: ServiceContainer,
{
    let sample = service_container.sample_service().find_sample(
        &query::FindSampleQuery { name: "nametest".into() }
    ).await;
    let sample = sample.unwrap();
    sample.name
}

pub fn routes<ServiceCt>(service_container: Arc<ServiceCt>) -> axum::Router
where
    ServiceCt: ServiceContainer + 'static,
{
    axum::Router::new()
        .route("/hello", axum::routing::get(hello::<ServiceCt>))
        .with_state(service_container)
}
