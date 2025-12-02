use anyhow::anyhow;

pub fn json_response(
    status_code: axum::http::StatusCode,
    json_body: String,
) -> axum::response::Response {
    axum::response::Response::builder()
        .status(status_code)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .header(axum::http::header::CONTENT_LENGTH, json_body.len())
        .body(axum::body::Body::from(json_body))
        .unwrap()
}

pub async fn into_bytes(body: axum::body::Body) -> crate::Result<bytes::Bytes> {
    Ok(axum::body::to_bytes(body, usize::MAX).await.map_err(|e| {
        crate::Error::Unhandled(anyhow!("to_bytes failed: {:?}", e))
    })?)
}

pub async fn into_string(body: axum::body::Body) -> crate::Result<String> {
    let bytes = into_bytes(body).await?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

pub fn is_json(headers: &axum::http::HeaderMap) -> bool {
    headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.starts_with("application/json"))
        .unwrap_or(false)
}
