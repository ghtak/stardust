use tower_sessions::Session;

pub const SESSION_COOKIE_NAME: &str = "x-session-id";
pub const SESEION_USER: &str = "x-session-user";

pub fn session_layer<S>(session_store: S) -> tower_sessions::SessionManagerLayer<S>
where
    S: tower_sessions::SessionStore,
{
    tower_sessions::SessionManagerLayer::new(session_store)
        .with_name(SESSION_COOKIE_NAME)
        .with_secure(false)
        .with_same_site(tower_sessions::cookie::SameSite::Lax)
}

pub async fn store_user<T>(session: &Session, user: &T) -> stardust_common::Result<()>
where
    T: serde::Serialize,
{
    session
        .insert(SESEION_USER, user)
        .await
        .map_err(|e| stardust_common::Error::StoreError(e.into()))?;
    Ok(())
}

pub async fn get_user<T>(session: &Session) -> stardust_common::Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    session.get(SESEION_USER).await.map_err(|e| stardust_common::Error::StoreError(e.into()))
}

pub async fn remove_user(session: &Session) -> stardust_common::Result<()> {
    session
        .remove_value(SESEION_USER)
        .await
        .map_err(|e| stardust_common::Error::StoreError(e.into()))?;
    Ok(())
}
