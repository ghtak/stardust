use crate::entity::{Role, Status};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]

pub struct UserAccountModel {
    pub uid: String,
    pub user_id: i64,
    pub account_type: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn create_table(
    handle: &mut stardust_db::Handle<'_>,
) -> stardust_common::Result<()> {
    sqlx::query(
        r#"
            create table if not exists stardust_user (
                id serial primary key,
                username varchar(255) not null,
                email varchar(255) not null,
                role varchar(255) not null,
                status varchar(255) not null,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
        "#,
    )
    .execute(handle.executor())
    .await
    .map_err(stardust_db::into_error)?;

    sqlx::query(
        r#"
            create table if not exists stardust_user_account (
                uid varchar(255) primary key,
                user_id integer not null,
                account_type varchar(255) not null,
                password_hash varchar(255) not null,
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
        "#,
    )
    .execute(handle.executor())
    .await
    .map_err(stardust_db::into_error)?;

    Ok(())
}
