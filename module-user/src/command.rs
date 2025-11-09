
pub enum SignupCommand{
    Local{
        username: String,
        email: String,
        password: String,
    },
    Provisioned{
        username: String,
        email: String,
        password: String,
    }
}
