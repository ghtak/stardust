use crate::{
    entity,
    infra::model::{self, UserAccountModel, UserModel},
    query,
};

pub async fn create_table(handle: &mut stardust_db::Handle<'_>) -> stardust_common::Result<()> {
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
    builder.push(" RETURNING id, username, email, role, status, created_at, updated_at");
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
    account_builder.push_values(std::iter::once(user_account_entity), |mut values, model| {
        values.push_bind(&model.uid);
        values.push_bind(&model.user_id);
        values.push_bind(model.account_type.to_string());
        values.push_bind(&model.password_hash);
        values.push_bind(model.created_at);
        values.push_bind(model.updated_at);
    });
    account_builder
        .push(" RETURNING uid, user_id, account_type, password_hash, created_at, updated_at");
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
    let mut builder = sqlx::QueryBuilder::new("SELECT * FROM stardust_user WHERE 1=1 ");
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
    let mut builder =
        sqlx::QueryBuilder::new("SELECT * FROM stardust_user_account WHERE user_id = ");
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
    let mut builder = sqlx::QueryBuilder::new(
        r#"
        SELECT
            u.id as user_id,
            u.username as user_username,
            u.email as user_email,
            u.role as user_role,
            u.status as user_status,
            u.created_at as user_created_at,
            u.updated_at as user_updated_at,
            ua.uid as account_uid,
            ua.user_id as account_user_id,
            ua.account_type as account_account_type,
            ua.password_hash as account_password_hash,
            ua.created_at as account_created_at,
            ua.updated_at as account_updated_at
        FROM stardust_user u
        LEFT JOIN stardust_user_account ua ON u.id = ua.user_id
        WHERE 1=1
    "#,
    );

    if let Some(id) = query.id {
        builder.push(" AND u.id = ");
        builder.push_bind(id);
    }

    if let Some(username) = query.username {
        builder.push(" AND u.username = ");
        builder.push_bind(username);
    }

    if let Some(email) = query.email {
        builder.push(" AND u.email = ");
        builder.push_bind(email);
    }

    if let Some(uid) = query.uid {
        builder.push(" AND ua.uid = ");
        builder.push_bind(uid);
    }

    let rows = builder
        .build_query_as::<model::UserAggregateModel>()
        .fetch_all(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;

    if rows.is_empty() {
        return Ok(None);
    }

    let mut aggregate: Option<entity::UserAggregate> = None;
    for r in rows {
        let agg = aggregate.get_or_insert_with(|| entity::UserAggregate {
            user: r.user_entity(),
            accounts: Vec::new(),
        });

        agg.accounts.push(r.account_entity());
    }

    Ok(aggregate)
}

pub async fn save_user_account(
    handle: &mut stardust_db::Handle<'_>,
    user_account_entity: &entity::UserAccountEntity,
) -> stardust_common::Result<entity::UserAccountEntity> {
    let mut builder = sqlx::QueryBuilder::new("UPDATE stardust_user_account SET password_hash = ");
    builder.push_bind(&user_account_entity.password_hash);
    builder.push(", updated_at = ");
    builder.push_bind(user_account_entity.updated_at);
    builder.push(" WHERE uid = ");
    builder.push_bind(&user_account_entity.uid);
    builder.push(" RETURNING *");
    let row = builder
        .build_query_as::<model::UserAccountModel>()
        .fetch_one(handle.executor())
        .await
        .map_err(stardust_db::into_error)?;
    Ok(row.into())
}
