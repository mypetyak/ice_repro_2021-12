use crate::proto::{fiddler_server::Fiddler, MyRequest, MyResponse};
use tonic::{async_trait, Request, Response, Status};

pub struct MyService {}

#[async_trait]
impl Fiddler for MyService {
    async fn prepare(&self, _req: Request<MyRequest>) -> Result<Response<MyResponse>, Status> {
        Ok(Response::new(MyResponse {}))
    }
}
