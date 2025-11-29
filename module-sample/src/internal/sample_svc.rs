use crate::{entity, query};

pub struct SampleServiceImpl {}

impl SampleServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl crate::service::SampleService for SampleServiceImpl {
    async fn find_sample(
        &self,
        query: &query::FindSampleQuery,
    ) -> stardust_common::Result<entity::HelloEntity> {
        Ok(entity::HelloEntity {
            name: query.name.clone(),
        })
    }
}
