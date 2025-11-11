#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
