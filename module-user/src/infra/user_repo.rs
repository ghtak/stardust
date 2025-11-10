use crate::{entity, infra::model, query};

pub async fn create_table(
    handle: &mut stardust_db::Handle<'_>,
) -> stardust_common::Result<()> {
    sqlx::query(
        r#"
            create table if not exists stardust_user (
                id BIGSERIAL PRIMARY KEY,
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
                user_id BIGINT not null,
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


pub async fn create_user(
    handle: &mut stardust_db::Handle<'_>,
    user_entity: &entity::UserEntity,
) -> stardust_common::Result<entity::UserEntity> {
    let mut builder = sqlx::QueryBuilder::new(
        "INSERT INTO stardust_user (username, email, role, status, created_at, updated_at) ",
    );
    builder.push_values(std::iter::once(user_entity), |mut values, model| {
        values.push_bind(&model.username);
        values.push_bind(&model.email);
        values.push_bind(model.role.to_string());
        values.push_bind(model.status.to_string());
        values.push_bind(model.created_at);
        values.push_bind(model.updated_at);
    });
    builder.push(
        " RETURNING id, username, email, role, status, created_at, updated_at",
    );
    let row = builder
        .build_query_as::<crate::infra::model::UserModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(row.into())
}

pub async fn create_user_account(
    handle: &mut stardust_db::Handle<'_>,
    user_account_entity: &entity::UserAccountEntity,
) -> stardust_common::Result<entity::UserAccountEntity> {
    let mut account_builder = sqlx::QueryBuilder::new(
        "INSERT INTO stardust_user_account (uid, user_id, account_type, password_hash, created_at, updated_at) ",
    );
    account_builder.push_values(
        std::iter::once(user_account_entity),
        |mut values, model| {
            values.push_bind(&model.uid);
            values.push_bind(&model.user_id);
            values.push_bind(model.account_type.to_string());
            values.push_bind(&model.password_hash);
            values.push_bind(model.created_at);
            values.push_bind(model.updated_at);
        },
    );
    account_builder.push(" RETURNING uid, user_id, account_type, password_hash, created_at, updated_at");
    let account_row = account_builder
        .build_query_as::<model::UserAccountModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(account_row.into())
}

pub async fn find_user(
    handle: &mut stardust_db::Handle<'_>,
    query: &query::FindUserQuery<'_>,
) -> stardust_common::Result<Option<entity::UserEntity>> {
    let mut builder =
        sqlx::QueryBuilder::new("SELECT * FROM stardust_user WHERE 1=1 ");
    if let Some(id) = query.id {
        builder.push(" AND id = ");
        builder.push_bind(id);
    }

    if let Some(username) = query.username {
        builder.push(" AND username = ");
        builder.push_bind(username);
    }

    if let Some(email) = query.email {
        builder.push(" AND email = ");
        builder.push_bind(email);
    }

    if let Some(uid) = query.uid {
        builder.push(" AND id IN (SELECT user_id FROM stardust_user_account WHERE uid = ");
        builder.push_bind(uid);
        builder.push(") ");
    }

    let Some(row) = builder
        .build_query_as::<model::UserModel>()
        .fetch_optional(handle.executor())
        .await
        .map_err(stardust_db::into_error)?
    else {
        return Ok(None);
    };
    Ok(Some(row.into()))
}

pub async fn find_user_accounts(
    handle: &mut stardust_db::Handle<'_>,
    user_id: i64,
) -> stardust_common::Result<Vec<crate::entity::UserAccountEntity>> {
    let mut builder = sqlx::QueryBuilder::new(
        "SELECT * FROM stardust_user_account WHERE user_id = ",
    );
    builder.push_bind(user_id);
    let rows = builder
        .build_query_as::<crate::infra::model::UserAccountModel>()
        .fetch_all(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    return Ok(rows.into_iter().map(Into::into).collect());
}

pub async fn find_user_aggregate(
    handle: &mut stardust_db::Handle<'_>,
    query: &crate::query::FindUserQuery<'_>,
) -> stardust_common::Result<Option<entity::UserAggregate>> {
    let Some(user) = find_user(handle, query).await? else {
        return Ok(None);
    };
    let accounts = find_user_accounts(handle, user.id).await?;
    Ok(Some(entity::UserAggregate { user, accounts }))
}