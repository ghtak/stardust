#[derive(Debug, Clone, Default)]
pub struct FindUserQuery<'a> {
    pub id: Option<i64>,
    pub uid: Option<&'a str>,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
}

impl<'a> FindUserQuery<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn by_id(id: i64) -> Self {
        Self {
            id: Some(id),
            ..Self::default()
        }
    }

    pub fn by_uid(uid: &'a str) -> Self {
        Self {
            uid: Some(uid),
            ..Self::default()
        }
    }

    pub fn by_username(username: &'a str) -> Self {
        Self {
            username: Some(username),
            ..Self::default()
        }
    }

    pub fn by_email(email: &'a str) -> Self {
        Self {
            email: Some(email),
            ..Self::default()
        }
    }

    pub fn with_id(mut self, id: i64) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_uid(mut self, uid: &'a str) -> Self {
        self.uid = Some(uid);
        self
    }

    pub fn with_username(mut self, username: &'a str) -> Self {
        self.username = Some(username);
        self
    }

    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }
}


pub struct FindApiKeyUserQuery<'a> {
    pub key_hash: &'a str,
}

pub struct GetApiKeysQuery {
    pub user_id: i64,
}