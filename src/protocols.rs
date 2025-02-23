use clap::Subcommand;

pub mod http;
mod tcp;
mod ws;

#[derive(Subcommand)]
pub(crate) enum Protocol {
    Http(http::HttpCommand),
    Tcp(tcp::TcpCommand),
    WebSocket(ws::WebSocketCommand),
}
