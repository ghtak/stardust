
pub struct FindOAuth2ClientQuery<'a>{
    pub client_id: Option<&'a str>
}

pub struct FindOAuth2AuthorizationQuery<'a>{
    pub auth_code_value: Option<&'a str>,
    pub refresh_token_hash: Option<&'a str>
}

pub struct FindOAuth2UserQuery<'a>{
    pub access_token: &'a str
}

