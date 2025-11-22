#[derive(Debug, Clone, Default)]
pub struct Database {}

impl Database {
    pub async fn new(_: &stardust_common::config::DatabaseConfig) -> stardust_common::Result<Self> {
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

    async fn tx_handle(&self) -> stardust_common::Result<Self::Handle<'_>> {
        Ok(Handle { db: self })
    }
}

impl<'a> crate::database::Handle for Handle<'a> {
    async fn commit(self) -> Result<(), stardust_common::Error> {
        Ok(())
    }

    async fn rollback(self) -> stardust_common::Result<()> {
        Ok(())
    }
}
