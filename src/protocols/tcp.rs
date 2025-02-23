use clap::Parser;

#[derive(Parser, Debug)]
pub(crate) struct TcpCommand {
    #[arg(help = "The TCP address to connect to")]
    pub(crate) address: String,
}