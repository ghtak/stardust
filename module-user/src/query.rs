pub struct FindUserQuery<'a> {
    pub id: Option<i64>,
    pub uid: Option<&'a str>,
    pub username: Option<&'a str>,
    pub email: Option<&'a str>,
}
