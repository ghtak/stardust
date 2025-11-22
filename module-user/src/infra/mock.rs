use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{entity, query};

pub struct MockUserRepository {
    pub user_store: Arc<Mutex<HashMap<i64, crate::entity::UserEntity>>>,
    pub account_store: Arc<Mutex<HashMap<i64, crate::entity::UserAccountEntity>>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            user_store: Arc::new(Mutex::new(HashMap::new())),
            account_store: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl crate::repository::UserRepository for MockUserRepository {
    type Handle<'h> = stardust_db::internal::mock::Handle<'h>;

    async fn create_user(
        &self,
        _handle: &mut Self::Handle<'_>,
        user_entity: &entity::UserEntity,
    ) -> stardust_common::Result<entity::UserEntity> {
        let mut user_store = self.user_store.lock().await;
        user_store.insert(user_entity.id, user_entity.clone());
        Ok(user_entity.clone())
    }

    async fn create_user_account(
        &self,
        _handle: &mut Self::Handle<'_>,
        _user_account_entity: &entity::UserAccountEntity,
    ) -> stardust_common::Result<entity::UserAccountEntity> {
        unimplemented!()
    }

    async fn find_user(
        &self,
        _handle: &mut Self::Handle<'_>,
        _query: &query::FindUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::UserEntity>> {
        unimplemented!()
    }

    async fn find_user_accounts(
        &self,
        _handle: &mut Self::Handle<'_>,
        _user_id: i64,
    ) -> stardust_common::Result<Vec<crate::entity::UserAccountEntity>> {
        unimplemented!()
    }

    async fn find_user_aggregate(
        &self,
        _handle: &mut Self::Handle<'_>,
        _query: &crate::query::FindUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::UserAggregate>> {
        unimplemented!()
    }

    async fn save_user_account(
        &self,
        _handle: &mut Self::Handle<'_>,
        _user_account_entity: &entity::UserAccountEntity,
    ) -> stardust_common::Result<entity::UserAccountEntity> {
        unimplemented!()
    }
}
