pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;
    pub type UserServiceImpl = module_user::internal::UserServiceImpl<HasherImpl>;
    pub type ApikeyServiceImpl = module_user::internal::ApikeyServiceImpl<HasherImpl>;
    pub type Container = crate::container::Container<UserServiceImpl, ApikeyServiceImpl>;
}

pub use dev_env::*;
