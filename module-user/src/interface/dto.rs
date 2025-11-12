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
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub uids: Vec<String>,
}
