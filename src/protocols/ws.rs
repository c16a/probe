use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct WebSocketCommand {
    #[arg(help = "The WebSocket URL to connect to")]
    url: String,
}