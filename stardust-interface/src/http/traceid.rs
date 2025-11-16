use axum::http::{HeaderName, HeaderValue, Request};
use axum::response::IntoResponse;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{Instrument, info_span};

const TRACE_ID: HeaderName = HeaderName::from_static("x-trace-id");
const TRACE_SPAN: &str = "trace";
//const DATETIME_FMT_STR: &str = "%Y/%m/%d %H:%M:%S";

fn generate_trace_id() -> String {
    let uid = stardust_common::utils::generate_uid_short();
    // let date =
    //     chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true);
    let date = chrono::Utc::now().timestamp_millis();
    format!("{uid}-{date}")
}

#[derive(Clone, Default)]
pub struct TraceIdLayer;

impl<S> Layer<S> for TraceIdLayer {
    type Service = TraceIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TraceIdService { inner }
    }
}

#[derive(Clone, Default)]
pub struct TraceIdService<S> {
    inner: S,
}

impl<B, S> Service<Request<B>> for TraceIdService<S>
where
    S: Service<Request<B>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Response: IntoResponse,
    B: Send + 'static,
{
    type Response = axum::response::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let trace_id = req
            .headers()
            .get(TRACE_ID)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_owned())
            .unwrap_or_else(generate_trace_id);
        let span = info_span!(TRACE_SPAN, trace_id = %trace_id);
        let mut inner = self.inner.clone();
        Box::pin(
            async move {
                let mut response = inner.call(req).await?.into_response();
                if let Ok(value) = HeaderValue::from_str(&trace_id) {
                    response.headers_mut().insert(TRACE_ID, value);
                }
                Ok(response)
            }
            .instrument(span),
        )
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_generate_trace_id() {
        print!("{}", super::generate_trace_id())
    }
}
