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

impl SignupCommand {
    pub fn username(&self) -> &str {
        match self {
            SignupCommand::Local { username, .. }
            | SignupCommand::Provisioned { username, .. } => username,
        }
    }

    pub fn email(&self) -> &str {
        match self {
            SignupCommand::Local { email, .. }
            | SignupCommand::Provisioned { email, .. } => email,
        }
    }
    pub fn password(&self) -> &str {
        match self {
            SignupCommand::Local { password, .. }
            | SignupCommand::Provisioned { password, .. } => password,
        }
    }
    pub fn account_type(&self) -> AccountType {
        match self {
            SignupCommand::Local { .. } => AccountType::Local,
            SignupCommand::Provisioned { account_type, .. } => {
                account_type.clone()
            }
        }
    }
    pub fn role(&self) -> Role {
        match self {
            SignupCommand::Local { .. } => Role::User,
            SignupCommand::Provisioned { role, .. } => role.clone(),
        }
    }
    pub fn status(&self) -> Status {
        match self {
            SignupCommand::Local { .. } => Status::Active,
            SignupCommand::Provisioned { status, .. } => status.clone(),
        }
    }
}

pub enum LoginCommand {
    Local { email: String, password: String },
}

pub struct CreateApiKeyCommand {
    pub user_id: i64,
    pub description: String,
}

pub struct DeactivateApiKeyCommand {
    pub request_user_id: i64,
    pub apikey_id: i64,
}

