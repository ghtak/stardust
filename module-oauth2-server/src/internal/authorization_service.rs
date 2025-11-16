use std::sync::Arc;

use crate::{command, entity, infra::authorization_repo, query, service};

pub struct OAuth2AuthorizationServiceImpl<H, CS> {
    database: stardust_db::Database,
    hasher: Arc<H>,
    oauth2_client_service: Arc<CS>,
}

impl<H, CS> OAuth2AuthorizationServiceImpl<H, CS>
where
    H: stardust_common::hash::Hasher,
    CS: service::OAuth2ClientService,
{
    pub fn new(
        database: stardust_db::Database,
        hasher: Arc<H>,
        oauth2_client_service: Arc<CS>,
    ) -> Self {
        Self {
            database,
            hasher,
            oauth2_client_service,
        }
    }
}

#[async_trait::async_trait]
impl<H, CS> service::OAuth2AuthorizationService for OAuth2AuthorizationServiceImpl<H, CS>
where
    H: stardust_common::hash::Hasher,
    CS: service::OAuth2ClientService,
{
    async fn verify(
        &self,
        command: &command::OAuth2VerifyCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2ClientEntity> {
        let clients = self
            .oauth2_client_service
            .find_clients(&query::FindOAuth2ClientQuery {
                client_id: Some(command.client_id),
            })
            .await?;

        if clients.len() == 0 {
            return Err(stardust_common::Error::NotFound);
        }

        let client = clients.first().unwrap();

        if !stardust_common::utils::contains(&client.redirect_uris, &command.redirect_uri) {
            return Err(stardust_common::Error::InvalidParameter(
                "Invalid Redirect uri".into(),
            ));
        }

        if !stardust_common::utils::contains(&client.scopes, &command.scope) {
            return Err(stardust_common::Error::InvalidParameter(
                "Invalid scope".into(),
            ));
        }

        Ok(client.clone())
    }

    async fn authorize(
        &self,
        command: &command::OAuth2AuthorizeCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2AuthorizationEntity> {
        let client = self.verify(&command.verify_command).await?;
        let authorization = entity::OAuth2AuthorizationEntity::new(
            client.id,
            command.principal.user.id,
            command.verify_command.scope.to_owned(),
            command.verify_command.state.to_owned(),
        );
        let auth =
            authorization_repo::create_authorization(&mut self.database.pool(), &authorization)
                .await?;
        Ok(auth)
    }
}
