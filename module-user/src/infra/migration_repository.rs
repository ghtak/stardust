// pub struct PostgresMigrationRepository {}

// impl PostgresMigrationRepository {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

// #[async_trait::async_trait]
// impl crate::repository::MigrationRepository for PostgresMigrationRepository {
//     type Handle<'h> = stardust_db::internal::postgres::Handle<'h>;

//     async fn create_user_store(
//         &self,
//         handle: &mut Self::Handle<'_>,
//     ) -> stardust_common::Result<()> {
//         sqlx::query(
//             r#"
//             create table if not exists stardust_user (
//                 id BIGSERIAL PRIMARY KEY,
//                 username varchar(255) not null,
//                 email varchar(255) not null,
//                 role varchar(255) not null,
//                 status varchar(255) not null,
//                 created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
//                 updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
//             );
//         "#,
//         )
//         .execute(handle.executor())
//         .await
//         .map_err(stardust_db::into_error)?;

//         sqlx::query(
//             r#"
//             create table if not exists stardust_user_account (
//                 uid varchar(255) primary key,
//                 user_id BIGINT not null,
//                 account_type varchar(255) not null,
//                 password_hash varchar(255) not null,
//                 created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
//                 updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
//             );
//         "#,
//         )
//         .execute(handle.executor())
//         .await
//         .map_err(stardust_db::into_error)?;

//         Ok(())
//     }

//     async fn create_apikey_store(
//         &self,
//         handle: &mut Self::Handle<'_>,
//     ) -> stardust_common::Result<()> {
//         sqlx::query(
//             r#"
//             create table if not exists stardust_apikey (
//                 id BIGSERIAL PRIMARY KEY,
//                 user_id BIGINT not null,
//                 key_hash varchar(255) not null,
//                 prefix varchar(255) not null,
//                 description varchar(255) not null,
//                 created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
//                 updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
//                 last_used_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
//                 deactivated_at TIMESTAMPTZ
//             );
//         "#,
//         )
//         .execute(handle.executor())
//         .await
//         .map_err(stardust_db::into_error)?;
//         Ok(())
//     }
// }
