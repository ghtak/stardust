use std::sync::Arc;

use crate::interface::ServiceProvider;

pub struct Container<UC, CS> {
    pub user_container: Arc<UC>,
    pub oauth2_client_service: Arc<CS>,
}

impl<UC, CS> Container<UC, CS>
where
    UC: module_user::interface::ServiceProvider,
    CS: crate::service::OAuth2ClientService,
{
    pub fn new(user_container: Arc<UC>, oauth2_client_service: Arc<CS>) -> Self {
        Self {
            user_container,
            oauth2_client_service,
        }
    }
}

impl<UC, CS> ServiceProvider for Container<UC, CS>
where
    UC: module_user::interface::ServiceProvider,
    CS: crate::service::OAuth2ClientService,
{
    type OAuth2ClientService = CS;

    fn oauth2_client_service(&self) -> Arc<Self::OAuth2ClientService> {
        self.oauth2_client_service.clone()
    }
}

impl<UC, CS> module_user::interface::ServiceProvider for Container<UC, CS>
where
    UC: module_user::interface::ServiceProvider,
    CS: crate::service::OAuth2ClientService,
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
