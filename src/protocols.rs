use clap::Subcommand;

pub mod http;
pub(crate) mod tcp;
pub mod ws;

#[derive(Subcommand)]
pub(crate) enum Protocol {
    Http(http::HttpCommand),
    Tcp(tcp::TcpCommand),
    WebSocket(ws::WebSocketCommand),
}
