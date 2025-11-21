use std::sync::Arc;

use crate::{
    command::{LoginCommand, SignupCommand},
    entity,
    query,
};

use stardust_db::database::Handle;

pub struct UserServiceImpl<Database, Hasher, UserRepo> {
    database: Database,
    hasher: Arc<Hasher>,
    user_repo: Arc<UserRepo>,
}

impl<Database, Hasher, UserRepo> UserServiceImpl<Database, Hasher, UserRepo>
where
    Database: stardust_db::database::Database,
    Hasher: stardust_common::hash::Hasher,
    UserRepo: for<'h> crate::repository::UserRepository<Handle<'h> = Database::Handle<'h>>,
{
    pub fn new(database: Database, hasher: Arc<Hasher>, user_repo: Arc<UserRepo>) -> Self {
        Self {
            database,
            hasher,
            user_repo,
        }
    }

    pub async fn rehash_password(&self, user_accout: &entity::UserAccountEntity, password: &str) {
        match self.hasher.hash(password) {
            Ok(hash) => {
                let mut save_user_account = user_accout.clone();
                save_user_account.password_hash = hash;
                if let Err(e) = self
                    .user_repo
                    .save_user_account(&mut self.database.handle(), &save_user_account)
                    .await
                {
                    tracing::warn!("failed to save user account: {:?}", e);
                }
            }
            Err(e) => {
                tracing::warn!("failed to rehash password: {:?}", e);
            }
        }
    }
}

#[async_trait::async_trait]
impl<Database, Hasher, UserRepo> crate::service::UserService for UserServiceImpl<Database, Hasher, UserRepo>
where
    Database: stardust_db::database::Database + 'static,
    Hasher: stardust_common::hash::Hasher,
    UserRepo: for<'h> crate::repository::UserRepository<Handle<'h> = Database::Handle<'h>>,
{
    async fn hello(&self) -> String {
        "hello".into()
    }

    async fn signup(
        &self,
        command: &SignupCommand,
    ) -> stardust_common::Result<entity::UserAggregate> {
        if let Some(user) = self.user_repo.find_user(
            &mut self.database.handle(),
            &crate::query::FindUserQuery::by_email(command.email()),
        )
        .await?
        {
            return Err(stardust_common::Error::Duplicate(Some(user.email)));
        }
        let mut handle = self.database.tx_handle().await?;
        let now = chrono::Utc::now();
        let user_entity = entity::UserEntity {
            id: 0,
            username: command.username().to_string(),
            email: command.email().to_string(),
            role: command.role(),
            status: command.status(),
            created_at: now,
            updated_at: now,
        };
        let user_entity = self.user_repo.create_user(&mut handle, &user_entity).await?;

        let password_hash = self.hasher.hash(command.password())?;
        let user_account_entity = entity::UserAccountEntity {
            uid: stardust_common::utils::generate_uid(),
            user_id: user_entity.id,
            account_type: command.account_type(),
            password_hash,
            created_at: now,
            updated_at: now,
        };
        let _account_entity =
            self.user_repo.create_user_account(&mut handle, &user_account_entity).await?;
        handle.commit().await?;
        stardust_core::audit(
            user_entity.id,
            "user.created",
            serde_json::json!(user_entity),
        );
        Ok(entity::UserAggregate {
            user: user_entity,
            accounts: vec![user_account_entity],
        })
    }

    async fn login(
        &self,
        command: &LoginCommand,
    ) -> stardust_common::Result<entity::UserAggregate> {
        match command {
            LoginCommand::Local { email, password } => {
                let query = query::FindUserQuery::by_email(email);
                let Some(user) =
                    self.user_repo.find_user_aggregate(&mut self.database.handle(), &query).await?
                else {
                    return Err(stardust_common::Error::Unauthorized);
                };
                for account in
                    user.accounts.iter().filter(|a| a.account_type == entity::AccountType::Local)
                {
                    let result = self.hasher.verify(password, &account.password_hash)?;
                    if result.is_valid == false {
                        continue;
                    }
                    if result.needs_rehash {
                        self.rehash_password(account, password).await;
                    }
                    return Ok(user);
                }
                Err(stardust_common::Error::Unauthorized)
            }
        }
    }
}
