pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;
    pub type UserServiceImpl = module_user::internal::UserServiceImpl<HasherImpl>;
    pub type ApikeyServiceImpl = module_user::internal::ApikeyServiceImpl<HasherImpl>;
    pub type UserContaierImpl =
        module_user::interface::container::Container<UserServiceImpl, ApikeyServiceImpl>;

    pub type OAuth2ClientServiceImpl =
        module_oauth2_server::internal::OAuth2ClientServiceImpl<HasherImpl>;

    pub type OAuth2AuthorizationServiceImpl =
        module_oauth2_server::internal::OAuth2AuthorizationServiceImpl<HasherImpl, OAuth2ClientServiceImpl>;

    pub type OAuth2ServerContainerImpl = module_oauth2_server::interface::container::Container<
        UserContaierImpl,
        OAuth2ClientServiceImpl,
        OAuth2AuthorizationServiceImpl,
    >;

    pub type Container = crate::container::Container<UserContaierImpl, OAuth2ServerContainerImpl>;
}

pub use dev_env::*;
