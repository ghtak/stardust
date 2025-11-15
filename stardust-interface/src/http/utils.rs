use axum::{
    body::{self, Body},
    http::HeaderMap,
};

pub fn is_json_content(headers: &HeaderMap) -> bool {
    headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.starts_with("application/json"))
        .unwrap_or(false)
}

pub async fn into_bytes(body: Body) -> stardust_common::Result<bytes::Bytes> {
    let bytes = body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| stardust_common::Error::ParseError(e.into()))?;
    Ok(bytes)
}

pub async fn into_string(body: Body) -> stardust_common::Result<String> {
    let bytes = into_bytes(body).await?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}
