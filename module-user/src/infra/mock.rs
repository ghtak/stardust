use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use crate::{entity, query};

pub struct MockUserRepository {
    pub user_store: Arc<Mutex<HashMap<i64, crate::entity::UserEntity>>>,
    pub account_store: Arc<Mutex<HashMap<String, crate::entity::UserAccountEntity>>>,
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
        user_account_entity: &entity::UserAccountEntity,
    ) -> stardust_common::Result<entity::UserAccountEntity> {
        let mut account_store = self.account_store.lock().await;
        account_store.insert(user_account_entity.uid.clone(), user_account_entity.clone());
        Ok(user_account_entity.clone())
    }

    async fn find_user(
        &self,
        _handle: &mut Self::Handle<'_>,
        q: &query::FindUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::UserEntity>> {
        let user_store = self.user_store.lock().await;
        for (k, v) in &*user_store {
            if let Some(id) = q.id {
                if id != *k {
                    continue;
                }
            }
            if let Some(username) = q.username {
                if username != v.username {
                    continue;
                }
            }
            if let Some(email) = q.email {
                if email != v.email {
                    continue;
                }
            }
            return Ok(Some(v.clone()));
        }

        if let Some(uid) = q.uid {
            let account_store = self.account_store.lock().await;
            if let Some(v) = account_store.get(uid) {
                if let Some(user) = user_store.get(&v.user_id) {
                    return Ok(Some(user.clone()));
                }
            }
        }
        Ok(None)
    }

    async fn find_user_accounts(
        &self,
        _handle: &mut Self::Handle<'_>,
        user_id: i64,
    ) -> stardust_common::Result<Vec<crate::entity::UserAccountEntity>> {
        let mut results = Vec::new();
        let account_store = self.account_store.lock().await;
        for (_,v) in &*account_store {
            if v.user_id == user_id {
                results.push(v.clone());
            }
        }
        Ok(results)
    }

    async fn find_user_aggregate(
        &self,
        _handle: &mut Self::Handle<'_>,
        _query: &crate::query::FindUserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::UserAggregate>> {
        if let Some(user) = self.find_user(_handle, _query).await? {
            let accounts = self.find_user_accounts(_handle, user.id).await?;
            return Ok(Some(entity::UserAggregate {
                user,
                accounts,
            }));
        }
        Ok(None)
    }

    async fn save_user_account(
        &self,
        _handle: &mut Self::Handle<'_>,
        user_account_entity: &entity::UserAccountEntity,
    ) -> stardust_common::Result<entity::UserAccountEntity> {
        let mut account_store = self.account_store.lock().await;
        let account = account_store.get_mut(&user_account_entity.uid).unwrap();
        *account = user_account_entity.clone();
        Ok(user_account_entity.clone())
    }
}
