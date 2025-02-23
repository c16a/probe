mod protocols;

use clap::Parser;
use tokio;

use protocols::Protocol;
#[derive(Parser)]
#[command(
    name = "probe",
    version = "1.0",
    author = "Chaitanya Munukutla",
    about = "A flexible curl-like CLI in Rust"
)]
struct Cli {
    #[command(subcommand)]
    protocol: Protocol,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.protocol {
        Protocol::Http(cmd) => protocols::http::handle_request(cmd).await,
        Protocol::WebSocket(cmd) => protocols::ws::handle_request(cmd).await,
        Protocol::Tcp(cmd) => protocols::tcp::handle_request(cmd).await,
    }
}
