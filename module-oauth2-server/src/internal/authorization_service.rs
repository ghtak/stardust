use std::{borrow::Cow, sync::Arc};

use crate::{command, entity, query, service};

pub struct OAuth2AuthorizationServiceImpl<
    Database,
    AuthorizationRepository,
    ClientService,
    Hasher,
> {
    database: Database,
    authorization_repo: Arc<AuthorizationRepository>,
    oauth2_client_service: Arc<ClientService>,
    hasher: Arc<Hasher>,
}

impl<Database, AuthorizationRepository, ClientService, Hasher>
    OAuth2AuthorizationServiceImpl<
        Database,
        AuthorizationRepository,
        ClientService,
        Hasher,
    >
where
    Database: stardust::database::Database + 'static,
    AuthorizationRepository: for<'h> crate::repository::AuthorizationRepository<
            Handle<'h> = Database::Handle<'h>,
        >,
    ClientService: service::OAuth2ClientService,
    Hasher: stardust::hash::Hasher,
{
    pub fn new(
        database: Database,
        authorization_repo: Arc<AuthorizationRepository>,
        oauth2_client_service: Arc<ClientService>,
        hasher: Arc<Hasher>,
    ) -> Self {
        Self {
            database,
            authorization_repo,
            oauth2_client_service,
            hasher,
        }
    }

    pub async fn issue_token(
        &self,
        command: &command::TokenCommand<'_>,
    ) -> stardust::Result<entity::OAuth2Token> {
        let Some(code) = command.code else {
            return Err(stardust::Error::InvalidParameter(
                "Invalid code".into(),
            ));
        };

        self.oauth2_client_service
            .verify(&command::VerifyOAuth2ClientCommand {
                client_id: command.client_id,
                client_secret: command.client_secret,
            })
            .await?;

        let Some(mut auth) = self
            .authorization_repo
            .find_authorization(
                &mut self.database.handle(),
                &query::FindOAuth2AuthorizationQuery {
                    auth_code_value: Some(code),
                    refresh_token_hash: None,
                    access_token: None,
                },
            )
            .await?
        else {
            return Err(stardust::Error::NotFound(Cow::Owned(code.to_owned())));
        };

        if auth.auth_code_expires_at < chrono::Utc::now() {
            return Err(stardust::Error::Unauthorized);
        }

        let access_token = stardust::utils::generate_uid();
        let refresh_token = stardust::utils::generate_uid();
        let refresh_token_hash = self.hasher.hash(&refresh_token).await?;
        auth.issue_token(access_token.clone(), refresh_token_hash);
        self.authorization_repo
            .save_authorization(&mut self.database.handle(), &auth)
            .await?;
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
    ) -> stardust::Result<entity::OAuth2Token> {
        let Some(refresh_token) = command.refresh_token else {
            return Err(stardust::Error::InvalidParameter(
                "Invalid refresh_token".into(),
            ));
        };

        self.oauth2_client_service
            .verify(&command::VerifyOAuth2ClientCommand {
                client_id: command.client_id,
                client_secret: command.client_secret,
            })
            .await?;

        let hash = self.hasher.hash(&refresh_token).await?;
        let Some(mut auth) = self
            .authorization_repo
            .find_authorization(
                &mut self.database.handle(),
                &query::FindOAuth2AuthorizationQuery {
                    auth_code_value: None,
                    refresh_token_hash: Some(&hash),
                    access_token: None,
                },
            )
            .await?
        else {
            return Err(stardust::Error::NotFound(Cow::Owned(
                refresh_token.to_owned(),
            )));
        };

        let access_token = stardust::utils::generate_uid();
        auth.refresh_token(access_token.clone());
        self.authorization_repo
            .save_authorization(&mut self.database.handle(), &auth)
            .await?;
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
impl<Database, AuthorizationRepository, ClientService, Hasher>
    service::OAuth2AuthorizationService
    for OAuth2AuthorizationServiceImpl<
        Database,
        AuthorizationRepository,
        ClientService,
        Hasher,
    >
where
    Database: stardust::database::Database + 'static,
    AuthorizationRepository: for<'h> crate::repository::AuthorizationRepository<
            Handle<'h> = Database::Handle<'h>,
        >,
    ClientService: service::OAuth2ClientService,
    Hasher: stardust::hash::Hasher,
{
    async fn verify(
        &self,
        command: &command::VerifyOAuth2AuthorizationCommand<'_>,
    ) -> stardust::Result<entity::OAuth2ClientEntity> {
        let clients = self
            .oauth2_client_service
            .find_clients(&query::FindOAuth2ClientQuery {
                client_id: Some(command.client_id),
            })
            .await?;

        if clients.len() == 0 {
            return Err(stardust::Error::NotFound(Cow::Owned(
                command.client_id.to_owned(),
            )));
        }

        let client = clients.first().unwrap();

        if !stardust::utils::contains(
            &client.redirect_uris,
            &command.redirect_uri,
        ) {
            return Err(stardust::Error::InvalidParameter(
                "Invalid Redirect uri".into(),
            ));
        }

        if !stardust::utils::contains(&client.scopes, &command.scope) {
            return Err(stardust::Error::InvalidParameter(
                "Invalid scope".into(),
            ));
        }

        Ok(client.clone())
    }

    async fn authorize(
        &self,
        command: &command::AuthorizeOAuth2Command<'_>,
    ) -> stardust::Result<entity::OAuth2AuthorizationEntity> {
        let client = self.verify(&command.verify_command).await?;
        let mut authorization = entity::OAuth2AuthorizationEntity::new(
            client.id,
            command.principal.id,
            command.verify_command.scope.to_owned(),
            command.verify_command.state.to_owned(),
        );
        if let Some(config) = &command.config {
            authorization.config = config.clone();
        }
        let auth = self
            .authorization_repo
            .create_authorization(&mut self.database.handle(), &authorization)
            .await?;
        Ok(auth)
    }

    async fn token(
        &self,
        command: &command::TokenCommand<'_>,
    ) -> stardust::Result<entity::OAuth2Token> {
        match command.grant_type {
            "authorization_code" => self.issue_token(&command).await,
            "refresh_token" => self.refresh_token(&command).await,
            _ => Err(stardust::Error::InvalidParameter(
                "Invalid grant_type".into(),
            )),
        }
    }

    async fn find_user(
        &self,
        query: &query::FindOAuth2UserQuery<'_>,
    ) -> stardust::Result<Option<entity::OAuthUserAggregate>> {
        self.authorization_repo
            .find_user(&mut self.database.handle(), &query)
            .await
    }

    async fn find_authorization(
        &self,
        query: &query::FindOAuth2AuthorizationQuery<'_>,
    ) -> stardust::Result<Option<entity::OAuth2AuthorizationEntity>> {
        self.authorization_repo
            .find_authorization(&mut self.database.handle(), &query)
            .await
    }
}
