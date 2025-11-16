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

    pub async fn issue_token(
        &self,
        command: &command::TokenCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2Token> {
        let Some(code) = command.code else {
            return Err(stardust_common::Error::InvalidParameter(
                "Invalid code".into(),
            ));
        };

        self.oauth2_client_service
            .verify(&command::VerifyOAuth2ClientCommand {
                client_id: command.client_id,
                client_secret: command.client_secret,
            })
            .await?;

        let Some(mut auth) = authorization_repo::find_authorization(
            &mut self.database.pool(),
            &query::FindOAuth2AuthorizationQuery {
                auth_code_value: Some(code),
                refresh_token_hash: None,
            },
        )
        .await?
        else {
            return Err(stardust_common::Error::NotFound);
        };

        if auth.auth_code_expires_at < chrono::Utc::now() {
            return Err(stardust_common::Error::Expired("code is expired".into()));
        }

        let access_token = stardust_common::utils::generate_uid();
        let refresh_token = stardust_common::utils::generate_uid();
        let refresh_token_hash = self.hasher.hash(&refresh_token)?;
        auth.issue_token(access_token.clone(), refresh_token_hash);
        authorization_repo::save_authorization(&mut self.database.pool(), &auth).await?;
        let token = entity::OAuth2Token {
            access_token,
            expires_in: 3600,
            refresh_token: Some(refresh_token),
            scope: auth.scope,
            token_type: "Bearer".into(),
        };
        Ok(token)
    }

    pub async fn refresh_token(
        &self,
        command: &command::TokenCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2Token> {
        let Some(refresh_token) = command.refresh_token else {
            return Err(stardust_common::Error::InvalidParameter(
                "Invalid refresh_token".into(),
            ));
        };

        self.oauth2_client_service
            .verify(&command::VerifyOAuth2ClientCommand {
                client_id: command.client_id,
                client_secret: command.client_secret,
            })
            .await?;

        let hash = self.hasher.hash(&refresh_token)?;
        let Some(mut auth) = authorization_repo::find_authorization(
            &mut self.database.pool(),
            &query::FindOAuth2AuthorizationQuery {
                auth_code_value: None,
                refresh_token_hash: Some(&hash),
            },
        )
        .await?
        else {
            return Err(stardust_common::Error::NotFound);
        };

        let access_token = stardust_common::utils::generate_uid();
        auth.refresh_token(access_token.clone());
        authorization_repo::save_authorization(&mut self.database.pool(), &auth).await?;
        let token = entity::OAuth2Token {
            access_token,
            expires_in: 3600,
            refresh_token: Some(refresh_token.to_owned()),
            scope: auth.scope,
            token_type: "Bearer".into(),
        };
        Ok(token)
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
        command: &command::VerifyOAuth2AuthorizationCommand<'_>,
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
        command: &command::AuthorizeOAuth2Command<'_>,
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

    async fn token(
        &self,
        command: &command::TokenCommand<'_>,
    ) -> stardust_common::Result<entity::OAuth2Token> {
        match command.grant_type {
            "authorization_code" => self.issue_token(&command).await,
            "refresh_token" => self.refresh_token(&command).await,
            _ => Err(stardust_common::Error::InvalidParameter(
                "Invalid grant_type".into(),
            )),
        }
    }

    async fn find_user(
        &self,
        query: &query::FindOAuth2UserQuery<'_>,
    ) -> stardust_common::Result<Option<entity::OAuthUserAggregate>> {
        authorization_repo::find_user(&mut self.database.pool(), &query).await
    }
}
