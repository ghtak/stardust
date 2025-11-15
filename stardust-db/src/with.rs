#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct With<T1, R1> {
    #[sqlx(flatten)]
    pub inner: T1,
    #[sqlx(flatten)]
    pub related: R1,
}

impl<T1, R1, T2, R2> From<With<T1, R1>> for stardust_common::With<T2, R2>
where
    T2: From<T1>,
    R2: From<R1>,
{
    fn from(value: With<T1, R1>) -> stardust_common::With<T2, R2> {
        stardust_common::With {
            inner: value.inner.into(),
            related: value.related.into(),
        }
    }
}
