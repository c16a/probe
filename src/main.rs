use clap::{Arg, ArgAction, Command, Subcommand};
use hyper::{Request, Uri, body::Bytes};
use hyper_tls::HttpsConnector;
use tokio;

use hyper_util::{client::legacy::Client, rt::TokioExecutor};

#[derive(Subcommand)]
enum Protocol {
    Http {
        #[arg(help = "The URL to request")]
        url: String,

        #[arg(
            short = 'X',
            long = "request",
            help = "HTTP method (GET, POST, etc.)",
            default_value = "GET"
        )]
        method: String,

        #[arg(short = 'd', long = "data", help = "Data to send in a POST request")]
        data: Option<String>,

        #[arg(long = "header", help = "Headers to include in the request", action = ArgAction::Append)]
        headers: Vec<String>,
    },
    Tcp {
        #[arg(help = "The TCP address to connect to")]
        address: String,
    },
    WebSocket {
        #[arg(help = "The WebSocket URL to connect to")]
        url: String,
    },
}

#[tokio::main]
async fn main() {
    let matches = Command::new("rcurl")
        .version("1.0")
        .author("Your Name")
        .about("A flexible curl-like CLI in Rust")
        .subcommand_required(true)
        .subcommand(
            Command::new("http")
                .about("Make an HTTP request")
                .arg(Arg::new("url").required(true).help("The URL to request"))
                .arg(
                    Arg::new("method")
                        .short('X')
                        .long("request")
                        .default_value("GET")
                        .help("HTTP method (GET, POST, etc.)"),
                )
                .arg(
                    Arg::new("data")
                        .short('d')
                        .long("data")
                        .help("Data to send in a POST request"),
                )
                .arg(
                    Arg::new("header")
                        .long("header")
                        .action(ArgAction::Append)
                        .help("Headers to include in the request"),
                ),
        )
        .subcommand(
            Command::new("tcp").about("Connect to a TCP address").arg(
                Arg::new("address")
                    .required(true)
                    .help("The TCP address to connect to"),
            ),
        )
        .subcommand(
            Command::new("websocket")
                .about("Connect to a WebSocket server")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("The WebSocket URL to connect to"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("http", sub_m)) => {
            let url = sub_m.get_one::<String>("url").unwrap();
            let method = sub_m.get_one::<String>("method").unwrap().to_uppercase();
            let data = sub_m.get_one::<String>("data");
            let headers = sub_m.get_many::<String>("header").unwrap_or_default();
            handle_http_request(url, &method, data, headers).await;
        }
        Some(("tcp", sub_m)) => {
            let address = sub_m.get_one::<String>("address").unwrap();
            println!("TCP connection to {} (not yet implemented)", address);
        }
        Some(("ws", sub_m)) => {
            let url = sub_m.get_one::<String>("url").unwrap();
            println!("WebSocket connection to {} (not yet implemented)", url);
        }
        _ => unreachable!("got {:?}", matches.subcommand()),
    }
}

async fn handle_http_request(
    url: &str,
    method: &str,
    data: Option<&String>,
    headers: impl Iterator<Item = &String>,
) {
    let https = HttpsConnector::new();
    let client = Client::builder(TokioExecutor::new()).build::<_, String>(https);

    let uri: Uri = url.parse().expect("Invalid URL");

    let mut req_builder = Request::builder().method(method).uri(uri);

    for header in headers {
        if let Some((key, value)) = header.split_once(": ") {
            req_builder = req_builder.header(key, value);
        }
    }

    let req = match method {
        "POST" | "PUT" | "PATCH" => req_builder.body(data.unwrap().into()).unwrap(),
        _ => req_builder.body(String::new()).unwrap(),
    };

    match client.request(req).await {
        Ok(res) => {
            println!("Response: {}", res.status());
            let body_bytes = Bytes::new();
            println!("Body: {}", String::from_utf8_lossy(&body_bytes));
        }
        Err(err) => eprintln!("Request failed: {}", err),
    }
}
