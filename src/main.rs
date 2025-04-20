use clap::{self, Parser};
use rmcp::transport::sse_server::SseServer;
mod functions;


#[derive(clap::Parser, Debug)]
pub struct Args {
    #[clap(long, default_value = "127.0.0.1", env = "MCP_STD_SERVER_HOST")]
    pub host: String,

    #[clap(short, long, default_value = "8000", env = "MCP_STD_SERVER_PORT")]
    pub port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let bind_address = format!("{}:{}", args.host, args.port);

    let ct = SseServer::serve(bind_address.parse().unwrap())
        .await.unwrap()
        .with_service(functions::Functions::new);

    tokio::signal::ctrl_c().await.unwrap();
    ct.cancel();
}