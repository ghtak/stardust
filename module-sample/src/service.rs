use crate::{entity, query};

#[async_trait::async_trait]
pub trait SampleService: Sync + Send {
    async fn find_sample(
        &self,
        query: &query::FindSampleQuery,
    ) -> stardust_common::Result<entity::HelloEntity>;
}
