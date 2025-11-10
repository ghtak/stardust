use std::str::FromStr;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum AccountType {
    Local,
    Provisioned,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserEntity {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserAccountEntity {
    pub uid: String,
    pub user_id: i64,
    pub account_type: AccountType,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserAggregate {
    pub user: UserEntity,
    pub accounts: Vec<UserAccountEntity>,
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Local => write!(f, "Local"),
            AccountType::Provisioned => write!(f, "Provisioned"),
        }
    }
}

impl FromStr for AccountType {
    type Err = stardust_common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Local" => Ok(AccountType::Local),
            "Provisioned" => Ok(AccountType::Provisioned),
            _ => Err(stardust_common::Error::ParseError(anyhow::anyhow!(
                "Invalid AccountType: {}",
                s
            ))),
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
        }
    }
}

impl FromStr for Role {
    type Err = stardust_common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(Role::Admin),
            "User" => Ok(Role::User),
            _ => Err(stardust_common::Error::ParseError(anyhow::anyhow!(
                "Invalid Role: {}",
                s
            ))),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Active => write!(f, "Active"),
            Status::Inactive => write!(f, "Inactive"),
        }
    }
}

impl FromStr for Status {
    type Err = stardust_common::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(Status::Active),
            "Inactive" => Ok(Status::Inactive),
            _ => Err(stardust_common::Error::ParseError(anyhow::anyhow!(
                "Invalid Status: {}",
                s
            ))),
        }
    }
}
