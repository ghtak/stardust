use crate::command;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<SignupRequest> for command::SignupCommand {
    fn from(value: SignupRequest) -> Self {
        command::SignupCommand::Local {
            username: value.username,
            email: value.email,
            password: value.password,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl From<LoginRequest> for command::LoginCommand {
    fn from(value: LoginRequest) -> Self {
        command::LoginCommand::Local {
            email: value.email,
            password: value.password,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub uids: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateApiKeyRequest {
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateApiKeyResponse {
    pub id: i64,
    pub key: String,
    pub description: String,
}


