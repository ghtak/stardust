pub const SESSION_COOKIE_NAME: &str = "x-session-id";

pub fn session_layer<S>(
    session_store: S,
) -> tower_sessions::SessionManagerLayer<S>
where
    S: tower_sessions::SessionStore,
{
    tower_sessions::SessionManagerLayer::new(session_store)
        .with_name(SESSION_COOKIE_NAME)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
}
