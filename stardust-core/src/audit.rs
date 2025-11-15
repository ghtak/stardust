
pub fn audit(actor_id: i64, event: &str, detail: serde_json::Value) {
    tracing::info!(
        target = "audit",
        actor_id = actor_id,
        event = event,
        detail = %detail
    );
}
