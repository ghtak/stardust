use std::sync::Arc;

use crate::interface::ServiceProvider;

pub struct Container<UC, CS, AS> {
    pub user_container: Arc<UC>,
    pub oauth2_client_service: Arc<CS>,
    pub oauth2_authorization_service: Arc<AS>,
}

impl<UC, CS, AS> Container<UC, CS, AS>
where
    UC: module_user::interface::ServiceProvider,
    CS: crate::service::OAuth2ClientService,
    AS: crate::service::OAuth2AuthorizationService,
{
    pub fn new(
        user_container: Arc<UC>,
        oauth2_client_service: Arc<CS>,
        oauth2_authorization_service: Arc<AS>,
    ) -> Self {
        Self {
            user_container,
            oauth2_client_service,
            oauth2_authorization_service,
        }
    }
}

impl<UC, CS, AS> ServiceProvider for Container<UC, CS, AS>
where
    UC: module_user::interface::ServiceProvider,
    CS: crate::service::OAuth2ClientService,
    AS: crate::service::OAuth2AuthorizationService,
{
    type OAuth2ClientService = CS;
    type OAuth2AuthorizationService = AS;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService> {
        self.oauth2_client_service.clone()
    }
    fn oauth2_authorization_service(&self) -> Arc<Self::OAuth2AuthorizationService> {
        self.oauth2_authorization_service.clone()
    }
}

impl<UC, CS, AS> module_user::interface::ServiceProvider for Container<UC, CS, AS>
where
    UC: module_user::interface::ServiceProvider,
    CS: crate::service::OAuth2ClientService,
    AS: crate::service::OAuth2AuthorizationService,
{
    type UserService = UC::UserService;
    type ApiKeyService = UC::ApiKeyService;

    fn user_service(&self) -> Arc<Self::UserService> {
        self.user_container.user_service()
    }

    fn apikey_service(&self) -> Arc<Self::ApiKeyService> {
        self.user_container.apikey_service()
    }
}
