use hello::proto::fiddler_server::FiddlerServer;
use hello::proto::server::MyService;
use tonic::transport::Server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(FiddlerServer::new(MyService {}))
        .serve("[::1]:6666".parse().unwrap())
        .await?;
    Ok(())
}
