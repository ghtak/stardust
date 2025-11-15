pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;
    pub type UserServiceImpl = module_user::internal::UserServiceImpl<HasherImpl>;
    pub type ApikeyServiceImpl = module_user::internal::ApikeyServiceImpl<HasherImpl>;
    pub type UserContaierImpl =
        module_user::interface::container::Container<UserServiceImpl, ApikeyServiceImpl>;


    pub type OAuth2ServerContainerImpl = module_oauth2_server::interface::container::Container;

    pub type Container = crate::container::Container<UserContaierImpl, OAuth2ServerContainerImpl>;
}

pub use dev_env::*;
