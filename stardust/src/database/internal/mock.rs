#[derive(Debug, Clone, Default)]
pub struct Database {}

impl Database {
    pub async fn new(_: &crate::config::DatabaseConfig) -> crate::Result<Self> {
        Ok(Self {})
    }
}

pub struct Handle<'a> {
    pub db: &'a Database,
}

impl crate::database::Database for Database {
    type Handle<'h>
        = Handle<'h>
    where
        Self: 'h;

    fn handle(&self) -> Self::Handle<'_> {
        Handle { db: self }
    }

    async fn tx_handle(&self) -> crate::Result<Self::Handle<'_>> {
        Ok(Handle { db: self })
    }
}

impl<'a> crate::database::Handle for Handle<'a> {
    async fn commit(self) -> Result<(), crate::Error> {
        Ok(())
    }

    async fn rollback(self) -> crate::Result<()> {
        Ok(())
    }
}
