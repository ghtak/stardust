use crate::entity::{AccountType, Role, Status};

pub enum SignupCommand {
    Local {
        username: String,
        email: String,
        password: String,
    },
    Provisioned {
        username: String,
        email: String,
        password: String,
        account_type: AccountType,
        role: Role,
        status: Status,
    },
}

pub enum LoginCommand{
    Local{
        email: String,
        password: String,
    }
}
