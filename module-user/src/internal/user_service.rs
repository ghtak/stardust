use std::sync::Arc;

use crate::{command::{LoginCommand, SignupCommand}, entity, infra::user_repo};

pub struct UserServiceImpl<Hasher> {
    database: stardust_db::Database,
    hasher: Arc<Hasher>,
}

impl<Hasher> UserServiceImpl<Hasher>
where
    Hasher: stardust_common::hash::Hasher,
{
    pub fn new(database: stardust_db::Database, hasher: Arc<Hasher>) -> Self {
        Self { database, hasher }
    }

    async fn create_internal_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        account_type: entity::AccountType,
        role: entity::Role,
        status: entity::Status,
    ) -> stardust_common::Result<entity::UserAggregate> {
        let mut handle = self.database.transaction().await?;
        let user_entity = entity::UserEntity {
            id: 0,
            username: username.to_string(),
            email: email.to_string(),
            role,
            status,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let user_entity =
            user_repo::create_user(&mut handle, &user_entity).await?;

        let password_hash = self.hasher.hash(password)?;
        let user_account_entity = entity::UserAccountEntity {
            uid: stardust_common::utils::generate_uid(),
            user_id: user_entity.id,
            account_type,
            password_hash,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let _account_entity =
            user_repo::create_user_account(&mut handle, &user_account_entity)
                .await?;
        handle.commit().await?;
        Ok(entity::UserAggregate {
            user: user_entity,
            accounts: vec![user_account_entity],
        })
    }
}

#[async_trait::async_trait]
impl<Hasher> crate::service::UserService for UserServiceImpl<Hasher>
where
    Hasher: stardust_common::hash::Hasher,
{
    async fn hello(&self) -> String {
        "hello".into()
    }

    async fn signup(
        &self,
        command: &SignupCommand,
    ) -> stardust_common::Result<entity::UserAggregate> {
        match command {
            SignupCommand::Local {
                username,
                email,
                password,
            } => {
                self.create_internal_user(
                    username,
                    email,
                    password,
                    crate::entity::AccountType::Local,
                    crate::entity::Role::User,
                    crate::entity::Status::Inactive,
                )
                .await
            }
            SignupCommand::Provisioned {
                username,
                email,
                password,
                account_type,
                role,
                status,
            } => {
                self.create_internal_user(
                    username,
                    email,
                    password,
                    account_type.clone(),
                    role.clone(),
                    status.clone(),
                )
                .await
            }
        }
    }

    async fn login(
        &self,
        command: &LoginCommand,
    ) -> stardust_common::Result<entity::UserAggregate>{
        match command {
            LoginCommand::Local { email, password } => {
                let query = crate::query::FindUserQuery {
                    id: None,
                    uid: None,
                    username: None,
                    email: Some(email),
                };
                let Some(user) = user_repo::find_user_aggregate(&mut self.database.pool(), &query).await? else {
                    return Err(stardust_common::Error::Unauthorized);
                };
                for account in user.accounts.iter() {
                    if account.account_type == crate::entity::AccountType::Local {
                        let result = self.hasher.verify(password, &account.password_hash)?;
                        if result.is_valid == false{
                            return Err(stardust_common::Error::Unauthorized)
                        }
                        // if result.needs_rehash {
                        //     let password_hash = self.hasher.hash(password)?;
                        // }
                        return Ok(user)
                    }
                }
                Err(stardust_common::Error::Unauthorized)
            }
        }
    }
}
