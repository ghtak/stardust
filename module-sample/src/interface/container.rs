use std::sync::Arc;

pub trait ServiceContainer: Sync + Send {
    type SampleService: crate::service::SampleService;

    fn sample_service(&self) -> Arc<Self::SampleService>;
}


pub struct Container<SampleSvc> {
    sample_service: Arc<SampleSvc>,
}

impl<SampleSvc> Container<SampleSvc>{
    pub fn new(sample_service: Arc<SampleSvc>) -> Self
    {
        Self {
            sample_service,
        }
    }
}

impl<SampleSvc> ServiceContainer for Container<SampleSvc>
where
    SampleSvc: crate::service::SampleService,
{
    type SampleService = SampleSvc;

    fn sample_service(&self) -> Arc<Self::SampleService> {
        self.sample_service.clone()
    }
}