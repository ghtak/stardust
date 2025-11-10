pub mod dev_env {

    pub type HasherImpl = stardust_common::hash::DummyHasher;
    pub type UserServiceImpl =
        module_user::internal::UserServiceImpl<HasherImpl>;
    pub type Container = crate::container::Container<UserServiceImpl>;
}

pub use dev_env::*;
