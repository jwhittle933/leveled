use crate::proto::{level_service_server::LevelService, LevelReply, LevelRequest};
use tonic::{Response as TonicResponse, Status};

#[derive(Default)]
pub struct LevelServiceImpl {}

impl LevelServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl LevelService for LevelServiceImpl {
    async fn query(
        &self,
        request: tonic::Request<LevelRequest>,
    ) -> Result<TonicResponse<LevelReply>, Status> {
        tracing::info!("Got a request from {:?}", request.remote_addr());

        let reply = LevelReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(TonicResponse::new(reply))
    }
}
