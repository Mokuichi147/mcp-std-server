use rmcp::transport::sse_server::SseServer;
mod functions;

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    let ct = SseServer::serve(BIND_ADDRESS.parse().unwrap())
        .await.unwrap()
        .with_service(functions::Functions::new);

    tokio::signal::ctrl_c().await.unwrap();
    ct.cancel();
}